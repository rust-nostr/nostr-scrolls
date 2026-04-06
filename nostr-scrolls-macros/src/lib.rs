// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

#![no_std]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![warn(clippy::large_futures)]
#![doc = include_str!("../../README.md")]

extern crate alloc;

use alloc::{string::ToString, vec::Vec};
use core::option::Option;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    FnArg, Ident, ItemFn, Pat, Type, Visibility,
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    token::{Comma, Mut},
};

/// Controls code generation for the `#[main]` macro.
struct MainAttrs {
    /// Disables the default panic handler, allowing a custom implementation.
    no_panic_handler: bool,
}

impl Parse for MainAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut no_panic_handler = false;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "no_panic_handler" => no_panic_handler = true,
                other => {
                    return Err(syn::Error::new(
                        ident.span(),
                        alloc::format!("unknown option: `{other}`"),
                    ));
                }
            }

            if input.peek(Comma) {
                input.parse::<Comma>()?;
            }
        }

        Ok(MainAttrs { no_panic_handler })
    }
}

/// Attribute macro that transforms a `run` function to read parameters from a
/// WASM memory.
///
/// Also initialize the panic handler to log panic messages
/// via `nostr_scrolls::log`. To disable this behavior, use
/// `#[nostr_scrolls::main(no_panic_handler)]`.
///
/// # Requirements
/// - The function must be named `run`
/// - All parameters must implement `nostr_scrolls::ReadParam`
///
/// # Parameters
///
/// The arguments in your `run` function must strictly match the order of the
/// parameters defined in your `kind:1227` event tags. The host provides them
/// sequentially in WASM memory.
///
/// ## Built-in Types
///
/// You can use the built-in types below, or implement your own by implementing
/// the `nostr_scrolls::ReadParam` trait. Wrap your Rust type in `Option` if
/// it's optional
///
/// | NIP-C5 Type  | Rust Type                         | Note                              |
/// |--------------|-----------------------------------|-----------------------------------|
/// | `public_key` | `nostr_scrolls::PublicKey`        |                                   |
/// | `event`      | `nostr_scrolls::Event`            |                                   |
/// | `string`     | [`&str`]                          |                                   |
/// | `number`     | [`i32`]                           |                                   |
/// | `timestamp`  | [`u32`]                           |                                   |
/// | `relay`      | [`&str`]                          | Validated by the host             |
///
/// # Example
/// ```rust
/// #[nostr_scrolls::main]
/// fn run(pkey: PublicKey, event: Event) {
///     // Your code here
/// }
/// ```
///
/// [`&str`]: str
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // Parse attributes
    let attrs = match syn::parse::<MainAttrs>(attr) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };

    // Ensure function name is 'run'
    if input_fn.sig.ident != "run" {
        return syn::Error::new(
            input_fn.sig.ident.span(),
            "function must be named `run` for `nostr_scrolls::main`",
        )
        .to_compile_error()
        .into();
    }

    // Check if the function is private
    if let Some(error) = ensure_run_is_private(&input_fn.vis) {
        return error;
    }

    // Check if the function is async
    if let Some(async_kw) = input_fn.sig.asyncness {
        return syn::Error::new(
            async_kw.span(),
            "`run` function must be synchronous for `nostr_scrolls::main`",
        )
        .to_compile_error()
        .into();
    }

    // Extract parameter names and types
    let params: Vec<(Option<Mut>, Ident, Type)> = input_fn
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    Some((
                        pat_ident.mutability,
                        pat_ident.ident.clone(),
                        (*pat_type.ty).clone(),
                    ))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Generate read statements for each parameter
    let read_statements: Vec<TokenStream2> = params
        .iter()
        .map(|(mutability, name, ty)| {
            quote! {
                let #mutability #name: #ty = <#ty as nostr_scrolls::ReadParam>::read_param(ptr, &mut offset);
            }
        })
        .collect();
    let body = input_fn.block;
    let fn_attrs = input_fn.attrs;

    // Check if the pointer is null
    let check_pointer = if read_statements.is_empty() {
        quote! {}
    } else {
        quote! {
            if ptr.is_null() {
                panic!("null pointer passed as a parameters pointer");
            }
        }
    };

    // Conditionally generate the panic handler
    let panic_handler = if attrs.no_panic_handler {
        quote! {}
    } else {
        quote! {
            #[panic_handler]
            fn panic(info: &core::panic::PanicInfo) -> ! {
                let msg = info.message().as_str().unwrap_or("panic occurred");

                _ = nostr_scrolls::log("PANIC!");
                _ = nostr_scrolls::log(msg);

                if let Some(location) = info.location() {
                    _ = nostr_scrolls::log(location.file());
                }

                core::arch::wasm32::unreachable()
            }
        }
    };

    // Generate the final expanded code
    let expanded = quote! {
        #panic_handler

        // The actual entry point that the host calls
        #(#fn_attrs )*
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn run(ptr: *const u8) {
            let mut offset = 0usize;

            #check_pointer

            // Read all parameters
            #(#read_statements)*

            // User function body
            #body
        }
    };

    TokenStream::from(expanded)
}

/// Ensure that the `run` function remains private, returning a compile error otherwise.
fn ensure_run_is_private(vis: &Visibility) -> Option<TokenStream> {
    let error_span = match vis {
        Visibility::Inherited => return None, // It's private, all good!
        Visibility::Public(pub_kw) => pub_kw.span(),
        Visibility::Restricted(vis_restricted) => vis_restricted.span(),
    };

    Some(
        syn::Error::new(
            error_span,
            "`run` function must be private for `nostr_scrolls::main`",
        )
        .to_compile_error()
        .into(),
    )
}
