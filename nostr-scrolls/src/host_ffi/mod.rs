// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

//! A safe wrapper around NIP-5C nostr API

pub(crate) mod safe_wrapper;
mod utils;

#[allow(dead_code)]
#[link(wasm_import_module = "nostr")]
unsafe extern "C" {
    /// Create a new empty request handle.
    fn req_new() -> i32;

    /// Add a pubkey to the `authors` filter. It must be a pointer to a 32-byte
    /// buffer.
    fn req_add_author(req: i32, pubkey_ptr: *const u8);

    /// Same as [`req_add_author`] but with a hex-encoded pubkey string. Length
    /// is assumed to be 64 characters.
    fn req_add_author_hex(req: i32, pubkey_hex_ptr: *const u8);

    /// Add an id to the `ids` filter. It must be a pointer to a 32-byte buffer.
    fn req_add_id(req: i32, id_ptr: *const u8);

    /// Same as [`req_add_id`] but with a hex-encoded id string. Length is
    /// assumed to be 64 characters.
    fn req_add_id_hex(req: i32, id_hex_ptr: *const u8);

    /// Add a kind integer to the `kinds` filter.
    fn req_add_kind(req: i32, kind: i32);

    /// Add a value to a tag filter. `tag` is the ASCII code of the letter (e.g.
    /// `112` is `"p"` and will be added to the filter as `"#p"`); `value_ptr/value_len`
    /// points to the tag value to match. The value is treated as a string.
    fn req_add_tag(req: i32, tag: i32, value_ptr: *const u8, value_len: i32);

    /// Same as [`req_add_tag`] but the value is a pointer to a 32-byte binary buffer that
    /// the host will convert to hex. `value_ptr` must point to a 32-byte buffer. This
    /// is useful for `#p`, `#e`, and other tag filters that need pubkey or event ID
    /// values.
    fn req_add_tag_bin32(req: i32, tag: i32, value_ptr: *const u8);

    /// Sets the `"limit"` attribute of the filter.
    fn req_set_limit(req: i32, limit: i32);

    /// Sets the `"since"` attribute of the filter.
    fn req_set_since(req: i32, timestamp: i32);

    /// Sets the `"until"` attribute of the filter.
    fn req_set_until(req: i32, timestamp: i32);

    /// Sets the `"search"` attribute of the filter.
    fn req_set_search(req: i32, ptr: *const u8, len: i32);

    /// Explicitly target a relay URL for this request.
    fn req_add_relay(req: i32, ptr: *const u8, len: i32);

    /// Mark this request to automatically close after `EOSE`.
    fn req_close_on_eose(req: i32);

    /// Sends the REQ to the target relays and returns a subscription handle.
    /// The request handle is consumed by this call — do not drop it separately.
    fn subscribe(req: i32) -> i32;

    /// Returns a pointer to a 32-byte buffer containing the event ID.
    fn event_get_id(event_handle: i32) -> *const u8;

    /// Returns a pointer to a 64-character string containing the hex event ID.
    fn event_get_id_hex(event_handle: i32) -> *const u8;

    /// Returns a pointer to a 32-byte buffer containing the public key of the
    /// event author.
    fn event_get_pubkey(event_handle: i32) -> *const u8;

    /// Returns a pointer to a 64-character string containing the hex public key
    /// of the event author.
    fn event_get_pubkey_hex(event_handle: i32) -> *const u8;

    /// Returns the kind integer directly.
    fn event_get_kind(event_handle: i32) -> i32;

    /// Returns the unix timestamp directly.
    fn event_get_created_at(event_handle: i32) -> i32;

    /// Returns a pointer to a buffer containing the event content.
    fn event_get_content(event_handle: i32) -> *const u8;

    /// Returns the total number of tags on this event.
    fn event_get_tag_count(event_handle: i32) -> i32;

    /// Returns the number of items in the tag at `tag_index`. `0` if no tag in
    /// the given index
    fn event_get_tag_item_count(event_handle: i32, tag_index: i32) -> i32;

    /// Returns a pointer to a buffer containing the item at `(tag_index,
    /// item_index)`. `0` if no tag or value in the given index
    fn event_get_tag_item(event_handle: i32, tag_index: i32, item_index: i32) -> *const u8;

    /// Same as `event_get_tag_item`, but returns a pointer to a 32-byte buffer
    /// of the item if it happened to be a pubkey or an event id; `0` if no tag or item
    /// in the given index or the value is not 64-byte hex string.
    fn event_get_tag_item_bin32(event_handle: i32, tag_index: i32, item_index: i32) -> *const u8;

    /// Finds the first tag whose name (item 0) matches the string at
    /// `name_ptr/name_len`, then returns a pointer to a buffer containing item
    /// `item_index` from that tag; `0` if no matching tag is found.
    fn event_get_tag_item_by_name(
        event_handle: i32,
        name_ptr: *const u8,
        name_len: i32,
        item_index: i32,
    ) -> *const u8;

    /// Same as `event_get_tag_item_by_name`, but returns a pointer to a 32-byte
    /// buffer of the value if it happened to be a pubkey or an event id;
    /// 0 otherwise.
    fn event_get_tag_item_by_name_bin32(
        event_handle: i32,
        name_ptr: *const u8,
        name_len: i32,
        item_index: i32,
    ) -> *const u8;

    /// Render an event through the client's native note renderer. The event
    /// handle is not consumed.
    fn display(event: i32);

    /// Emit a log message to the host's debug console or developer tooling. The
    /// string at `ptr/len` is the message.
    fn log(ptr: *const u8, len: i32);

    /// Releases any handle: unconsumed filter, event or active subscription
    /// (cancels it)
    pub(crate) fn drop(handle: i32);
}
