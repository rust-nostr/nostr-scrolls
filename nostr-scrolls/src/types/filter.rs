// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::{Error, EventId, IntoHandle, PublicKey, Result, Subscription, host_ffi::safe_wrapper};

/// Nostr scrolls filter
#[derive(Debug)]
pub struct Filter {
    /// The filter handle in the host
    pub(crate) handle: i32,
    /// whenever if [`Filter::close_on_eose`] was called
    pub(crate) close_on_eose: bool,
}

impl IntoHandle for Filter {
    fn handle(&self) -> i32 {
        self.handle
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
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
    pub fn author(&mut self, pkey: &PublicKey) {
        safe_wrapper::req_add_author(self, pkey);
    }

    /// Add an author by 64-character hexadecimal public key.
    #[inline(always)]
    pub fn author_hex(&mut self, pkey: &str) -> Result<()> {
        safe_wrapper::req_add_author_hex(self, pkey)
    }

    /// Add an event ID to match against.
    #[inline(always)]
    pub fn id(&mut self, event_id: &EventId) {
        safe_wrapper::req_add_id(self, event_id);
    }

    /// Add an event ID by 64-character hexadecimal string.
    #[inline(always)]
    pub fn id_hex(&mut self, event_id: &str) -> Result<()> {
        safe_wrapper::req_add_id_hex(self, event_id)
    }

    /// Include events of a specific kind in the filter.
    #[inline(always)]
    pub fn kind(&mut self, kind: u16) {
        safe_wrapper::req_add_kind(self, kind);
    }

    /// Include events matching a single-letter tag with a string value.
    ///
    /// Returns `Error::SizeOverflow` if `value` length exceeds `i32::MAX`.
    /// `tag` must be an ASCII alphabetic character; returns `Error::InvalidTag`
    /// otherwise.
    #[inline(always)]
    pub fn tag(&mut self, tag: char, value: &str) -> Result<()> {
        if !tag.is_ascii_alphabetic() {
            return Err(Error::InvalidTag);
        }
        safe_wrapper::req_add_tag(self, tag, value)
    }

    /// Include events matching a single-letter tag with a fixed-size binary value.
    ///
    /// `bytes` must be exactly 32 bytes. `tag` must be an ASCII alphabetic
    /// character; returns `Error::InvalidTag` otherwise.
    #[inline(always)]
    pub fn tag_bytes(&mut self, tag: char, bytes: &[u8]) -> Result<()> {
        if !tag.is_ascii_alphabetic() {
            return Err(Error::InvalidTag);
        }
        safe_wrapper::req_add_tag_bin32(self, tag, bytes)
    }

    /// Limits the number of events returned by the filter.
    ///
    /// Errors if `limit` exceeds [`i32::MAX`].
    #[inline(always)]
    pub fn limit(&mut self, limit: usize) -> Result<()> {
        safe_wrapper::req_set_limit(self, limit)
    }

    /// Only return events created after this timestamp.
    ///
    /// Errors if `since` exceeds [`i32::MAX`].
    #[inline(always)]
    pub fn since(&mut self, since: usize) -> Result<()> {
        safe_wrapper::req_set_since(self, since)
    }

    /// Only return events created before this timestamp.
    ///
    /// Errors if `until` exceeds [`i32::MAX`].
    #[inline(always)]
    pub fn until(&mut self, until: usize) -> Result<()> {
        safe_wrapper::req_set_until(self, until)
    }

    /// Filters events by content substring match.
    ///
    /// Errors if `search` byte length exceeds [`i32::MAX`].
    #[inline(always)]
    pub fn search(&mut self, search: &str) -> Result<()> {
        safe_wrapper::req_set_search(self, search)
    }

    /// Adds a relay target for this subscription, can be called multiple time.
    ///
    /// Errors if `relay` byte length exceeds [`i32::MAX`].
    #[inline(always)]
    pub fn send_to(&mut self, relay: &str) -> Result<()> {
        safe_wrapper::req_add_relay(self, relay)
    }

    /// Closes the subscription automatically when the relay finishes sending stored events.
    #[inline(always)]
    pub fn close_on_eose(&mut self) {
        safe_wrapper::req_close_on_eose(self);
        self.close_on_eose = true;
    }

    /// Consumes the filter and initiates the subscription.
    #[inline]
    pub fn subscribe(self) -> Subscription {
        safe_wrapper::subscribe(self)
    }
}
