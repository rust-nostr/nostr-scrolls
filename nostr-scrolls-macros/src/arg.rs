// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use alloc::string::ToString;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, FnArg, Ident, Pat, Path, Token, Type, spanned::Spanned};

/// Scroll argument attributes
#[derive(Default)]
struct ArgAtrrs {
    from_ty: Option<Path>,
}

impl ArgAtrrs {
    /// Parse argument attributes to [`ArgAtrrs`]
    fn parse(arg_ident: &Ident, attrs: &[Attribute]) -> syn::Result<Self> {
        let mut this = Self::default();
        let mut found_from = None;

        for attr in attrs {
            match attr.path().require_ident()?.to_string().as_str() {
                "from" => {
                    check_multiple_use_attr(found_from, attr.span(), "from", arg_ident)?;
                    found_from = Some(attr.span());
                    this.parse_from(attr)?;
                }
                other => {
                    return Err(syn::Error::new(
                        attr.path().span(),
                        alloc::format!("Unknown attr `{other}`"),
                    ));
                }
            }
        }
        Ok(this)
    }

    fn parse_from(&mut self, attr: &Attribute) -> syn::Result<()> {
        self.from_ty = Some(attr.parse_args::<Path>()?);

        Ok(())
    }
}

/// A function argument
pub struct ScrollArg {
    attrs: ArgAtrrs,
    mutability: Option<Token![mut]>,
    name: Ident,
    ty: Type,
}

impl From<ScrollArg> for TokenStream {
    fn from(arg: ScrollArg) -> Self {
        let ScrollArg {
            attrs: ArgAtrrs { from_ty },
            mutability,
            name,
            ty,
        } = arg;

        if let Some(from_ty) = from_ty {
            quote! {
                let #mutability #name: #ty = <#ty as core::convert::From<#from_ty>>::from(<#from_ty as nostr_scrolls::ReadParam>::read_param(ptr, &mut offset));
            }
        } else {
            quote! {
                let #mutability #name: #ty = <#ty as nostr_scrolls::ReadParam>::read_param(ptr, &mut offset);
            }
        }
    }
}

impl ScrollArg {
    /// Scroll arg from function arg
    pub fn from_fn_arg(arg: &FnArg) -> syn::Result<Option<Self>> {
        if let FnArg::Typed(pat_type) = arg
            && let Pat::Ident(ident) = pat_type.pat.as_ref()
        {
            let arg_ident = ident.ident.clone();
            return Ok(Some(Self {
                attrs: ArgAtrrs::parse(&arg_ident, &pat_type.attrs)?,
                mutability: ident.mutability,
                name: arg_ident,
                ty: pat_type.ty.as_ref().clone(),
            }));
        }
        Ok(None)
    }
}

fn check_multiple_use_attr(
    old_span: Option<Span>,
    new_span: Span,
    attr_name: &str,
    arg_ident: &Ident,
) -> syn::Result<()> {
    if let Some(span) = old_span {
        let mut err = syn::Error::new(
            new_span,
            alloc::format!("Multiple `{attr_name}` attr found for `{arg_ident}` argument"),
        );
        err.combine(syn::Error::new(span, "First one is here"));

        return Err(err);
    }
    Ok(())
}
