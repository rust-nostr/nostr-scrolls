// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

//! Nostr API types

mod event;
mod filter;
mod numbers;
mod static_cell;
mod subscription;

use core::slice;

pub use event::Event;
pub use filter::*;
pub use numbers::*;
pub use static_cell::*;
pub use subscription::Subscription;

use crate::{ReadParam, utils};

/// Nostr scrolls public key
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "debug-strings", derive(core::fmt::Debug))]
pub struct PublicKey(pub(crate) [u8; 32]);

/// Nostr scrolls event id
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "debug-strings", derive(core::fmt::Debug))]
pub struct EventId(pub(crate) [u8; 32]);

impl<'a> ReadParam<'a> for PublicKey {
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self {
        if !utils::read_presence_flag(ptr, offset) {
            panic!("ReadParam(public_key): Expected required parameter, but host provided 0x00");
        }

        let mut buf = [0u8; 32];
        buf.copy_from_slice(slice::from_raw_parts(ptr.add(*offset), 32));

        *offset += 32;
        PublicKey(buf)
    }
}
