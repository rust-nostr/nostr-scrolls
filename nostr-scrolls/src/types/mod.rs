// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

//! Nostr API types

mod event;
mod filter;
mod subscription;

pub use event::Event;
pub use filter::Filter;
pub use subscription::Subscription;

use crate::{ReadParam, utils};

/// Nostr scrolls public key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicKey(pub(crate) [u8; 32]);

/// Nostr scrolls event id
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventId(pub(crate) [u8; 32]);

impl<'a, 'b> ReadParam<'a, 'b> for PublicKey {
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self {
        if !utils::read_presence_flag(cursor, buffer) {
            panic!("ReadParam(PublicKey): Expected required parameter, but host provided 0x00");
        }

        if buffer.len() < *cursor + 32 {
            panic!(
                "ReadParam(PublicKey): Out of bounds reading string payload (expected 32 bytes, remaining {})",
                buffer.len() - *cursor
            );
        }

        let mut buf = [0u8; 32];
        buf.copy_from_slice(&buffer[*cursor..*cursor + 32]);
        *cursor += 32;

        PublicKey(buf)
    }
}

impl<'a, 'b> ReadParam<'a, 'b> for Option<PublicKey> {
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self {
        if !utils::peek_presence_flag(cursor, buffer) {
            *cursor += 1;
            return None;
        }

        Some(<PublicKey as ReadParam>::read_param(cursor, buffer))
    }
}
