// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::{
    EventId, PublicKey, Subscription,
    host_ffi::{drop as ffi_drop, safe_wrapper},
};

/// Nostr scrolls filter
#[cfg_attr(feature = "debug-strings", derive(core::fmt::Debug))]
pub struct Filter {
    /// The filter handle in the host
    pub(crate) handle: i32,
    /// whenever if [`Filter::close_on_eose`] was called
    pub(crate) close_on_eose: bool,
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        unsafe { ffi_drop(self.handle) }
    }
}

impl Filter {
    /// Create a new empty filter.
    #[inline(always)]
    pub fn new() -> Self {
        safe_wrapper::req_new()
    }

    /// Add an author by public key.
    #[inline(always)]
    pub fn author(self, pkey: &PublicKey) -> Self {
        safe_wrapper::req_add_author(&self, pkey);
        self
    }

    /// Require events to be authored by the 64-character hex public key.
    ///
    /// ## Panics
    /// Panics if `pkey.len() != 64`.
    #[inline(always)]
    pub fn author_hex(self, pkey: &str) -> Self {
        safe_wrapper::req_add_author_hex(&self, pkey);
        self
    }

    /// Add an event ID to match against.
    #[inline(always)]
    pub fn id(self, event_id: &EventId) -> Self {
        safe_wrapper::req_add_id(&self, event_id);
        self
    }

    /// Add an event ID by 64-character hexadecimal string.
    ///
    /// ## Panics
    /// Panics if `event_id.len() != 64`.
    #[inline(always)]
    pub fn id_hex(self, event_id: &str) -> Self {
        safe_wrapper::req_add_id_hex(&self, event_id);
        self
    }

    /// Include events of a specific kind in the filter.
    #[inline(always)]
    pub fn kind(self, kind: u16) -> Self {
        safe_wrapper::req_add_kind(&self, kind);
        self
    }

    /// Include events matching a single-letter tag with a string value.
    ///
    /// # Panics
    /// Panic if the given tag is not ASCII alphabetic
    #[inline(always)]
    pub fn tag(self, tag: char, value: &str) -> Self {
        assert!(tag.is_ascii_alphabetic());
        safe_wrapper::req_add_tag(&self, tag, value);
        self
    }

    /// Include events matching a single-letter tag with a fixed-size binary value.
    ///
    /// # Panics
    /// Panics if `tag` is not ASCII alphabetic or `bytes` is not exactly 32 bytes
    #[inline(always)]
    #[doc(alias = "add_tag_bin32")]
    pub fn tag_bytes(self, tag: char, bytes: &[u8]) -> Self {
        assert!(tag.is_ascii_alphabetic());
        safe_wrapper::req_add_tag_bin32(&self, tag, bytes);
        self
    }

    /// Limits the number of events returned by the filter.
    #[inline(always)]
    pub fn limit(self, limit: usize) -> Self {
        safe_wrapper::req_set_limit(&self, limit);
        self
    }

    /// Only return events created after this timestamp.
    #[inline(always)]
    pub fn since(self, since: usize) -> Self {
        safe_wrapper::req_set_since(&self, since);
        self
    }

    /// Only return events created before this timestamp.
    #[inline(always)]
    pub fn until(self, until: usize) -> Self {
        safe_wrapper::req_set_until(&self, until);
        self
    }

    /// Filters events by content substring match.
    #[inline(always)]
    pub fn search(self, search: &str) -> Self {
        safe_wrapper::req_set_search(&self, search);
        self
    }

    /// Adds a relay target for this subscription, can be called multiple time.
    #[inline(always)]
    #[doc(alias = "add_relay")]
    pub fn send_to(self, relay: &str) -> Self {
        safe_wrapper::req_add_relay(&self, relay);
        self
    }

    /// Closes the subscription automatically when the relay finishes sending stored events.
    #[inline(always)]
    pub fn close_on_eose(mut self) -> Self {
        safe_wrapper::req_close_on_eose(&self);
        self.close_on_eose = true;
        self
    }

    /// Consumes the filter and initiates the subscription.
    #[inline]
    pub fn subscribe(self) -> Subscription {
        safe_wrapper::subscribe(self)
    }
}
