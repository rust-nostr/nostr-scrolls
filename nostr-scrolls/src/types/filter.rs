// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::{
    EventId, PublicKey, StaticCell, Subscription,
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

/// A filter that can be used as a static variable.
///
/// Calling any function after consuming the filter in
/// [`StaticFilter::subscribe`] function will create a new filter, because the
/// old one is dropped
#[cfg_attr(feature = "debug-strings", derive(core::fmt::Debug))]
pub struct StaticFilter(StaticCell<Option<Filter>>);

impl Default for StaticFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl StaticFilter {
    /// Create a new static filter
    #[inline]
    pub const fn new() -> Self {
        Self(StaticCell::new(None))
    }

    /// Take the current filter or initialize a new one.
    #[inline(never)]
    fn take_filter(&self) -> Filter {
        unsafe {
            self.0
                .get_mut()
                .take()
                .or_else(|| Some(Filter::new()))
                .unwrap_unchecked()
        }
    }

    /// Replace the current filter with a new one.
    #[inline]
    fn set_filter(&self, filter: Filter) {
        (*self.0.get_mut()) = Some(filter)
    }

    /// A static wrapper around [`Filter::author`]
    #[inline]
    pub fn author(&self, pkey: &PublicKey) {
        self.set_filter(self.take_filter().author(pkey))
    }

    /// A static wrapper around [`Filter::author_hex`]
    #[inline]
    pub fn author_hex(&self, pkey: &str) {
        self.set_filter(self.take_filter().author_hex(pkey));
    }

    /// A static wrapper around [`Filter::id`]
    #[inline]
    pub fn id(&self, event_id: &EventId) {
        self.set_filter(self.take_filter().id(event_id));
    }

    /// A static wrapper around [`Filter::id_hex`]
    #[inline]
    pub fn id_hex(&self, event_id: &str) {
        self.set_filter(self.take_filter().id_hex(event_id));
    }

    /// A static wrapper around [`Filter::kind`]
    #[inline]
    pub fn kind(&self, kind: u16) {
        self.set_filter(self.take_filter().kind(kind));
    }

    /// A static wrapper around [`Filter::tag`]
    #[inline]
    pub fn tag(&self, tag: char, value: &str) {
        self.set_filter(self.take_filter().tag(tag, value));
    }

    /// A static wrapper around [`Filter::tag_bytes`]
    #[inline]
    #[doc(alias = "add_tag_bin32")]
    pub fn tag_bytes(&self, tag: char, bytes: &[u8]) {
        self.set_filter(self.take_filter().tag_bytes(tag, bytes));
    }

    /// A static wrapper around [`Filter::limit`]
    #[inline]
    pub fn limit(&self, limit: usize) {
        self.set_filter(self.take_filter().limit(limit));
    }

    /// A static wrapper around [`Filter::since`]
    #[inline]
    pub fn since(&self, since: usize) {
        self.set_filter(self.take_filter().since(since));
    }

    /// A static wrapper around [`Filter::until`]
    #[inline]
    pub fn until(&self, until: usize) {
        self.set_filter(self.take_filter().until(until));
    }

    /// A static wrapper around [`Filter::search`]
    #[inline]
    pub fn search(&self, search: &str) {
        self.set_filter(self.take_filter().search(search));
    }

    /// A static wrapper around [`Filter::send_to`]
    #[inline]
    #[doc(alias = "add_relay")]
    pub fn send_to(&self, relay: &str) {
        self.set_filter(self.take_filter().send_to(relay));
    }

    /// A static wrapper around [`Filter::close_on_eose`]
    #[inline]
    pub fn close_on_eose(&self) {
        self.set_filter(self.take_filter().close_on_eose())
    }

    /// A static wrapper around [`Filter::subscribe`].
    ///
    /// This will consume the filter, any other call to the static filter
    /// functions will create a new filter
    #[inline]
    pub fn subscribe(&self) -> Subscription {
        self.take_filter().subscribe()
    }
}
