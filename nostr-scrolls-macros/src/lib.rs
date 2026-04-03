// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

#![no_std]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![warn(clippy::large_futures)]
#![doc = include_str!("../../README.md")]

extern crate alloc;

use alloc::vec::Vec;
use core::option::Option;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{FnArg, Ident, ItemFn, Pat, Type, Visibility, parse_macro_input, spanned::Spanned};

/// Attribute macro that transforms a `run` function to read parameters from a
/// WASM buffer.
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
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

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
    let params: Vec<(Ident, Type)> = input_fn
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    Some((pat_ident.ident.clone(), (*pat_type.ty).clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Get parameter names and types for code generation
    let param_names: Vec<&Ident> = params.iter().map(|(name, _)| name).collect();

    // Generate read statements for each parameter
    let read_statements: Vec<TokenStream2> = params
        .iter()
        .map(|(name, ty)| {
            quote! {
                let #name: #ty = <#ty as nostr_scrolls::ReadParam>::read_param(&mut cursor, &buffer);
            }
        })
        .collect();

    // Generate parameter list for function call
    let call_params: Vec<TokenStream2> = param_names.iter().map(|name| quote! { #name }).collect();

    // Rename the user's function
    let user_fn_ident = format_ident!("__nostr_scrolls_user_run");
    let mut user_fn = input_fn.clone();
    user_fn.sig.ident = user_fn_ident.clone();

    // Generate the final expanded code
    let expanded = quote! {
        // The user's original function, renamed
        #user_fn

        // The actual entry point that the host calls
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn run(ptr: *const u8, len: i32) {
            let len = len as usize;

            // Create slice from WASM linear memory
            let buffer = unsafe { core::slice::from_raw_parts(ptr, len) };
            let mut cursor = 0usize;

            // Read all parameters from the buffer
            #(#read_statements)*

            // Verify all bytes were consumed
            if cursor != buffer.len() {
                panic!(
                    "nostr_scrolls: Parameter buffer length mismatch - consumed {} bytes, but buffer has {} bytes",
                    cursor,
                    buffer.len()
                );
            }

            // Call the user's function with the extracted parameters
            #user_fn_ident(#(#call_params),*);
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
