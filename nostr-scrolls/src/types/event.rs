// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::{EventId, IntoHandle, PublicKey, ReadParam, Result, host_ffi::safe_wrapper};

/// Nostr scrolls event
pub struct Event {
    /// The event handle in the host
    pub(crate) handle: i32,
}

impl IntoHandle for Event {
    fn handle(&self) -> i32 {
        self.handle
    }
}

impl<'a, 'b> ReadParam<'a, 'b> for Event {
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self {
        Self {
            handle: <i32 as ReadParam>::read_param(cursor, buffer),
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        crate::drop(Event {
            handle: self.handle,
        });
    }
}

impl Event {
    /// Returns the event ID.
    #[inline(always)]
    pub fn id(&self) -> EventId {
        safe_wrapper::event_get_id(self)
    }

    /// Returns the event ID as a lowercase hexadecimal string.
    #[inline(always)]
    pub fn id_hex(&self) -> &str {
        safe_wrapper::event_get_id_hex(self)
    }

    /// Returns the public key of the event's author.
    #[inline(always)]
    pub fn pubkey(&self) -> PublicKey {
        safe_wrapper::event_get_pubkey(self)
    }

    /// Returns the author's public key as a lowercase hexadecimal string.
    #[inline(always)]
    pub fn pubkey_hex(&self) -> &str {
        safe_wrapper::event_get_pubkey_hex(self)
    }

    /// Returns the event kind.
    #[inline(always)]
    pub fn kind(&self) -> u16 {
        safe_wrapper::event_get_kind(self)
    }

    /// Returns the Unix timestamp when this event was created.
    #[inline(always)]
    pub fn created_at(&self) -> usize {
        safe_wrapper::event_get_created_at(self)
    }

    /// Returns the event's content.
    #[inline(always)]
    pub fn content(&self) -> &str {
        safe_wrapper::event_get_content(self)
    }

    /// Total number of tags present in this event.
    #[inline(always)]
    pub fn tag_count(&self) -> usize {
        safe_wrapper::event_get_tag_count(self)
    }

    /// Number of items in a specific tag by index.
    ///
    /// Returns `None` when the tag index is out of bounds.
    /// Error if `tag_index` exceeds `i32::MAX`.
    #[inline(always)]
    pub fn tag_items_count(&self, tag_index: usize) -> Result<Option<usize>> {
        safe_wrapper::event_get_tag_item_count(self, tag_index)
    }

    /// String value of a specific tag item by index.
    ///
    /// Returns `None` when either index is out of bounds.
    /// Error if `tag_index` exceeds `i32::MAX`.
    #[inline(always)]
    pub fn tag_item(&self, tag_index: usize, item_index: usize) -> Result<Option<&str>> {
        safe_wrapper::event_get_tag_item(self, tag_index, item_index)
    }

    /// Decoded bytes of a 64-byte hex tag item by index.
    ///
    /// Returns `None` when either index is out of bounds, or if the value is
    /// not valid hexadecimal or not exactly 64 bytes. Error if `tag_index`
    /// exceeds `i32::MAX`.
    #[inline(always)]
    pub fn tag_item_bytes(&self, tag_index: usize, item_index: usize) -> Result<Option<&[u8]>> {
        safe_wrapper::event_get_tag_item_bin32(self, tag_index, item_index)
    }

    /// String value of a tag item by tag name.
    ///
    /// Returns `None` if no tag with this name exists or the item index is out
    /// of bounds. Error if `tag_index` exceeds `i32::MAX`.
    #[inline(always)]
    pub fn tag_item_by_name(&self, name: &str, item_index: usize) -> Result<Option<&str>> {
        safe_wrapper::event_get_tag_item_by_name(self, name, item_index)
    }

    /// Decoded bytes of a 64-byte hex tag item by tag name.
    ///
    /// Returns `None` if no tag with this name exists, the item index is out of
    /// bounds, or the value is not valid hexadecimal or not exactly 64 bytes.
    /// Error if `tag_index` exceeds `i32::MAX`.
    #[inline(always)]
    pub fn tag_item_by_name_bytes(&self, name: &str, item_index: usize) -> Result<Option<&[u8]>> {
        safe_wrapper::event_get_tag_item_by_name_bin32(self, name, item_index)
    }
}
