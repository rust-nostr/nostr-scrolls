// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

//! A safe wrapper around nostr API

use core::str;

use crate::{Event, EventId, Filter, PublicKey, Subscription, host_ffi::utils};

const BYTES_LEN: usize = 32;
const HEX_LEN: usize = 64;

/// A wrapper around [`super::req_new`]. Used by [`Filter`]
#[inline(always)]
pub(crate) fn req_new() -> Filter {
    unsafe {
        Filter {
            handle: super::req_new(),
            close_on_eose: false,
        }
    }
}

/// A wrapper around [`super::req_add_author`]. Used by [`Filter`]
#[inline(always)]
pub(crate) fn req_add_author(filter: &Filter, pubkey: &PublicKey) {
    unsafe { super::req_add_author(filter.handle, pubkey.0.as_ptr()) }
}

/// A wrapper around [`super::req_add_author_hex`]. Used by [`Filter`].
///
/// - The hex should be 64 bytes string
#[inline(always)]
pub(crate) fn req_add_author_hex(filter: &Filter, pubkey_hex: &str) {
    assert_eq!(pubkey_hex.len(), HEX_LEN);

    unsafe { super::req_add_author_hex(filter.handle, pubkey_hex.as_ptr()) }
}

/// A wrapper around [`super::req_add_id`]. Used by [`Filter`]
#[inline(always)]
pub(crate) fn req_add_id(filter: &Filter, event_id: &EventId) {
    unsafe { super::req_add_id(filter.handle, event_id.0.as_ptr()) }
}

/// A wrapper around [`super::req_add_id_hex`]. Used by [`Filter`]
///
/// - The hex should be 64 bytes string
#[inline(always)]
pub(crate) fn req_add_id_hex(filter: &Filter, id_hex: &str) {
    assert_eq!(id_hex.len(), HEX_LEN);

    unsafe { super::req_add_id_hex(filter.handle, id_hex.as_ptr()) }
}

/// A wrapper around [`super::req_add_kind`]. Used by [`Filter`]
#[inline(always)]
pub(crate) fn req_add_kind(filter: &Filter, kind: u16) {
    unsafe { super::req_add_kind(filter.handle, kind) }
}

/// A wrapper around [`super::req_add_tag`]. Used by [`Filter`]
#[inline(always)]
pub(crate) fn req_add_tag(filter: &Filter, tag: char, value: &str) {
    // Note: You can't allocate more than `isize::MAX`, which is `i32::MAX` in
    // `wasm32`
    unsafe {
        super::req_add_tag(
            filter.handle,
            tag as i32,
            value.as_ptr(),
            value.len() as i32,
        )
    }
}

/// A wrapper around [`super::req_add_tag_bin32`]. Used by [`Filter`].
///
/// - The `tag` should be an ASCII alphabetic character
/// - The value should be 32 bytes
#[inline(always)]
pub(crate) fn req_add_tag_bin32(filter: &Filter, tag: char, value: &[u8]) {
    assert_eq!(value.len(), BYTES_LEN);

    unsafe { super::req_add_tag_bin32(filter.handle, tag as i32, value.as_ptr()) }
}

/// A wrapper around [`super::req_set_limit`]. Used by [`Filter`].
#[inline(always)]
pub(crate) fn req_set_limit(filter: &Filter, limit: usize) {
    unsafe { super::req_set_limit(filter.handle, limit as u32) }
}

/// A wrapper around [`super::req_set_since`]. Used by [`Filter`].
#[inline(always)]
pub(crate) fn req_set_since(filter: &Filter, timestamp: usize) {
    unsafe { super::req_set_since(filter.handle, timestamp as u32) }
}

/// A wrapper around [`super::req_set_until`]. Used by [`Filter`].
#[inline(always)]
pub(crate) fn req_set_until(filter: &Filter, timestamp: usize) {
    unsafe { super::req_set_until(filter.handle, timestamp as u32) }
}

/// A wrapper around [`super::req_set_search`]. Used by [`Filter`].
#[inline(always)]
pub(crate) fn req_set_search(filter: &Filter, search: &str) {
    // Note: You can't allocate more than `isize::MAX`, which is `i32::MAX` in
    // `wasm32`
    unsafe { super::req_set_search(filter.handle, search.as_ptr(), search.len() as i32) }
}

/// A wrapper around [`super::req_add_relay`]. Used by [`Filter`].
#[inline(always)]
pub(crate) fn req_add_relay(filter: &Filter, relay: &str) {
    // Note: You can't allocate more than `isize::MAX`, which is `i32::MAX` in
    // `wasm32`
    unsafe { super::req_add_relay(filter.handle, relay.as_ptr(), relay.len() as i32) }
}

/// A wrapper around [`super::req_close_on_eose`]. Used by [`Filter`].
#[inline(always)]
pub(crate) fn req_close_on_eose(filter: &Filter) {
    unsafe { super::req_close_on_eose(filter.handle) }
}

/// A wrapper around [`super::subscribe`]. Used by [`Filter`].
#[inline(always)]
pub(crate) fn subscribe(filter: Filter) -> Subscription {
    unsafe {
        Subscription {
            handle: super::subscribe(filter.handle),
            close_on_eose: filter.close_on_eose,
        }
    }
}

// -- Event -- //

/// A wrapper around [`super::event_get_id`]. Used by [`Event`].
#[inline(always)]
pub(crate) fn event_get_id(event: &Event) -> EventId {
    unsafe { EventId(utils::read_slice_owned(super::event_get_id(event.handle))) }
}

/// A wrapper around [`super::event_get_id_hex`]. Used by [`Event`].
#[inline(always)]
pub(crate) fn event_get_id_hex(event: &Event) -> &str {
    unsafe {
        str::from_utf8_unchecked(utils::read_slice(
            super::event_get_id_hex(event.handle),
            HEX_LEN,
        ))
    }
}

/// A wrapper around [`super::event_get_pubkey`]. Used by [`Event`].
#[inline(always)]
pub(crate) fn event_get_pubkey(event: &Event) -> PublicKey {
    unsafe {
        PublicKey(utils::read_slice_owned(super::event_get_pubkey(
            event.handle,
        )))
    }
}

/// A wrapper around [`super::event_get_pubkey_hex`]. Used by [`Event`].
#[inline(always)]
pub(crate) fn event_get_pubkey_hex(event: &Event) -> &str {
    unsafe {
        str::from_utf8_unchecked(utils::read_slice(
            super::event_get_pubkey_hex(event.handle),
            HEX_LEN,
        ))
    }
}

/// A wrapper around [`super::event_get_kind`]. Used by [`Event`].
#[inline(always)]
pub(crate) fn event_get_kind(event: &Event) -> u16 {
    unsafe { super::event_get_kind(event.handle) as u16 }
}

/// A wrapper around [`super::event_get_created_at`]. Used by [`Event`].
#[inline(always)]
pub(crate) fn event_get_created_at(event: &Event) -> usize {
    unsafe { super::event_get_created_at(event.handle) as usize }
}

/// A wrapper around [`super::event_get_content`]. Used by [`Event`].
#[inline(always)]
pub(crate) fn event_get_content(event: &Event) -> &str {
    unsafe { utils::read_slice_string(super::event_get_content(event.handle)) }
}

/// A wrapper around [`super::event_get_tag_count`]. Used by [`Event`].
#[inline(always)]
pub(crate) fn event_get_tag_count(event: &Event) -> usize {
    unsafe { super::event_get_tag_count(event.handle) as usize }
}

/// A wrapper around [`super::event_get_tag_item_count`]. Used by [`Event`].
///
/// - Returns [`Option::None`] if there is no tag in the given index
#[inline(always)]
pub(crate) fn event_get_tag_item_count(event: &Event, tag_index: usize) -> Option<usize> {
    unsafe {
        let count = super::event_get_tag_item_count(event.handle, tag_index as u32) as usize;
        if count == 0 {
            return None;
        }
        Some(count)
    }
}

/// A wrapper around [`super::event_get_tag_item`]. Used by [`Event`].
///
/// - Return [`Option::None`] if no tag or item with the given index
#[inline(always)]
pub(crate) fn event_get_tag_item(
    event: &Event,
    tag_index: usize,
    item_index: usize,
) -> Option<&str> {
    unsafe {
        let ptr = super::event_get_tag_item(event.handle, tag_index as u32, item_index as u32);
        if ptr.is_null() {
            return None;
        }
        Some(utils::read_slice_string(ptr))
    }
}

/// A wrapper around [`super::event_get_tag_item_bin32`]. Used by [`Event`].
///
/// - Returns [`Option::None`] if:
///   - No tag in the given index
///   - No item in the given index
///   - The item is invalid 64-byte hex string
#[inline(always)]
pub(crate) fn event_get_tag_item_bin32(
    event: &Event,
    tag_index: usize,
    item_index: usize,
) -> Option<&[u8]> {
    unsafe {
        let ptr =
            super::event_get_tag_item_bin32(event.handle, tag_index as u32, item_index as u32);
        if ptr.is_null() {
            return None;
        }

        Some(utils::read_slice(ptr, 32))
    }
}

/// A wrapper around [`super::event_get_tag_item_by_name`]. Used by [`Event`].
///
/// - Returns [`Option::None`] if:
///   - No tag with the given name. (item index 0)
///   - No item in the given index
#[inline(always)]
pub(crate) fn event_get_tag_item_by_name<'a>(
    event: &'a Event,
    name: &str,
    item_index: usize,
) -> Option<&'a str> {
    // Note: You can't allocate more than `isize::MAX`, which is `i32::MAX` in
    // `wasm32`
    unsafe {
        let ptr = super::event_get_tag_item_by_name(
            event.handle,
            name.as_ptr(),
            name.len() as i32,
            item_index as u32,
        );

        if ptr.is_null() {
            return None;
        }

        Some(utils::read_slice_string(ptr))
    }
}

/// A wrapper around [`super::event_get_tag_item_by_name_bin32`]. Used by [`Event`].
///
/// - Returns [`Option::None`] if:
///   - No tag with the given name. (item index 0)
///   - No item in the given index
///   - The item is invalid 64-byte hex string
#[inline(always)]
pub(crate) fn event_get_tag_item_by_name_bin32<'a>(
    event: &'a Event,
    name: &str,
    item_index: usize,
) -> Option<&'a [u8]> {
    unsafe {
        let ptr = super::event_get_tag_item_by_name_bin32(
            event.handle,
            name.as_ptr(),
            name.len() as i32,
            item_index as u32,
        );

        if ptr.is_null() {
            return None;
        }

        Some(utils::read_slice(ptr, 32))
    }
}

// -- Public -- //

/// Render an event through the client's native note renderer.
#[inline(always)]
pub fn display(event: &Event) {
    unsafe {
        super::display(event.handle);
    }
}

/// Emit a log message to the host's debug console or developer tooling.
#[inline(always)]
pub fn log(msg: &str) {
    unsafe { super::log(msg.as_ptr(), msg.len() as i32) }
}
