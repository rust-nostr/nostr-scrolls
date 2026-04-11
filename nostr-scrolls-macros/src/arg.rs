// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, Pat, Token, Type};

/// A function argument
pub struct ScrollArg {
    mutability: Option<Token![mut]>,
    name: Ident,
    ty: Type,
}

impl TryFrom<&FnArg> for ScrollArg {
    type Error = ();

    fn try_from(arg: &FnArg) -> Result<Self, Self::Error> {
        if let FnArg::Typed(pat_type) = arg
            && let Pat::Ident(ident) = pat_type.pat.as_ref()
        {
            return Ok(Self {
                mutability: ident.mutability,
                name: ident.ident.clone(),
                ty: pat_type.ty.as_ref().clone(),
            });
        }
        Err(())
    }
}

impl From<ScrollArg> for TokenStream {
    fn from(arg: ScrollArg) -> Self {
        let ScrollArg {
            mutability,
            name,
            ty,
        } = arg;

        quote! {
            let #mutability #name: #ty = <#ty as nostr_scrolls::ReadParam>::read_param(ptr, &mut offset);
        }
    }
}
