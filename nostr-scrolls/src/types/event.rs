// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::{
    EventId, PublicKey, ReadParam,
    host_ffi::{drop as ffi_drop, safe_wrapper},
    inner_utils,
};

/// Nostr scrolls event
pub struct Event {
    /// The event handle in the host
    pub(crate) handle: i32,
}

impl<'a> ReadParam<'a> for Event {
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self {
        if !inner_utils::read_presence_flag(ptr, offset) {
            panic!("ReadParam(event): Expected required parameter, but host provided 0x00");
        }

        Self {
            handle: inner_utils::read_i32(ptr, offset),
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { ffi_drop(self.handle) }
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
    #[inline(always)]
    pub fn tag_items_count(&self, tag_index: usize) -> Option<usize> {
        safe_wrapper::event_get_tag_item_count(self, tag_index)
    }

    /// String value of a specific tag item by index.
    ///
    /// Returns `None` when either index is out of bounds.
    #[inline(always)]
    pub fn tag_item(&self, tag_index: usize, item_index: usize) -> Option<&str> {
        safe_wrapper::event_get_tag_item(self, tag_index, item_index)
    }

    /// Decoded bytes of a 64-byte hex tag item by index.
    ///
    /// Returns `None` when either index is out of bounds, or if the value is
    /// not valid hexadecimal or not exactly 64 bytes.
    #[inline(always)]
    #[doc(alias = "get_tag_item_bin32")]
    pub fn tag_item_bytes(&self, tag_index: usize, item_index: usize) -> Option<[u8; 32]> {
        safe_wrapper::event_get_tag_item_bin32(self, tag_index, item_index)
    }

    /// String value of a tag item by tag name.
    ///
    /// Returns `None` if no tag with this name exists or the item index is out
    /// of bounds.
    #[inline(always)]
    pub fn tag_item_by_name(&self, name: &str, item_index: usize) -> Option<&str> {
        safe_wrapper::event_get_tag_item_by_name(self, name, item_index)
    }

    /// Decoded bytes of a 64-byte hex tag item by tag name.
    ///
    /// Returns `None` if no tag with this name exists, the item index is out of
    /// bounds, or the value is not valid hexadecimal or not exactly 64 bytes.
    /// Error if `tag_index` exceeds `i32::MAX`.
    #[inline(always)]
    #[doc(alias = "get_tag_item_by_name_bin32")]
    pub fn tag_item_by_name_bytes(&self, name: &str, item_index: usize) -> Option<[u8; 32]> {
        safe_wrapper::event_get_tag_item_by_name_bin32(self, name, item_index)
    }
}
