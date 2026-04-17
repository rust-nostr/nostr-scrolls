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

use crate::{ReadParam, inner_utils};

/// Short public key for indexing. First and last 4 bytes
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "debug-strings", derive(core::fmt::Debug))]
pub struct ShortPubKey(pub(crate) [u8; 8]);

/// Nostr scrolls public key
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "debug-strings", derive(core::fmt::Debug))]
pub struct PublicKey(pub(crate) [u8; 32]);

impl<'a> ReadParam<'a> for PublicKey {
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self {
        if !inner_utils::read_presence_flag(ptr, offset) {
            panic!("ReadParam(public_key): Expected required parameter, but host provided 0x00");
        }

        let mut buf = [0u8; 32];
        buf.copy_from_slice(slice::from_raw_parts(ptr.add(*offset), 32));

        *offset += 32;
        PublicKey(buf)
    }
}

impl PublicKey {
    /// Derives a compact identifier for the public key
    pub fn short(&self) -> ShortPubKey {
        let mut buf = [0u8; 8];
        buf[..4].copy_from_slice(&self.0[..4]);
        buf[4..].copy_from_slice(&self.0[28..]);
        ShortPubKey(buf)
    }

    /// Check if the public key match the short one
    pub fn matches_short(&self, short: &ShortPubKey) -> bool {
        self.0[..4] == short.0[..4] && self.0[28..] == short.0[4..]
    }
}

/// Short event id for indexing. First and last 4 bytes
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "debug-strings", derive(core::fmt::Debug))]
pub struct ShortEventId(pub(crate) [u8; 8]);

/// Nostr scrolls event id
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "debug-strings", derive(core::fmt::Debug))]
pub struct EventId(pub(crate) [u8; 32]);

impl EventId {
    /// Derives a compact identifier for the event id
    pub fn short(&self) -> ShortEventId {
        let mut buf = [0u8; 8];
        buf[..4].copy_from_slice(&self.0[..4]);
        buf[4..].copy_from_slice(&self.0[28..]);
        ShortEventId(buf)
    }

    /// Check if the event id match the short one
    pub fn matches_short(&self, short: &ShortEventId) -> bool {
        self.0[..4] == short.0[..4] && self.0[28..] == short.0[4..]
    }
}
