// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

//! Utils to help you in your program

use alloc::vec::Vec;

use crate::Event;

/// Relay marker
#[repr(C)]
pub enum Relay<'a> {
    /// Write relay
    Write(&'a str),
    /// Read relay
    Read(&'a str),
    /// Relay for write and read
    Both(&'a str),
}

impl Relay<'_> {
    /// Returns the relay url
    #[inline]
    pub fn url(&self) -> &str {
        match self {
            Self::Write(relay) => relay,
            Self::Read(relay) => relay,
            Self::Both(relay) => relay,
        }
    }

    /// Check if the relay is WRITE relay
    #[inline]
    pub fn is_write(&self) -> bool {
        matches!(self, Self::Write(_) | Self::Both(_))
    }

    /// Check if the relay is READ relay
    #[inline]
    pub fn is_read(&self) -> bool {
        matches!(self, Self::Read(_) | Self::Both(_))
    }
}

/// Read the relays from `10002` event. (NIP-65)
pub fn read_relays(event: &Event, limit: usize) -> Vec<Relay<'_>> {
    if event.kind() != 10002 {
        return Vec::new();
    }

    let mut relays = Vec::with_capacity(limit);
    unsafe {
        for tag_idx in 0..(event.tag_count().min(limit)) {
            if event.tag_items_count(tag_idx).unwrap_unchecked() < 2 {
                continue;
            }

            let relay = event.tag_item(tag_idx, 1).unwrap_unchecked();
            let marker = event.tag_item(tag_idx, 2);

            match marker {
                Some("write") => relays.push(Relay::Write(relay)),
                Some("read") => relays.push(Relay::Read(relay)),
                None | Some("") => relays.push(Relay::Both(relay)),
                // Unknown marker
                _ => {}
            }
        }
    }
    relays
}
