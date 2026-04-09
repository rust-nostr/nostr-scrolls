// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::Event;

/// Determines whether a value is present by inspecting the byte at the given
/// offset.
///
/// # Panics
///
/// Panics if the presence byte is neither `0x00` nor `0x01`.
pub(crate) unsafe fn peek_presence_flag(ptr: *const u8, offset: usize) -> bool {
    let flag = *ptr.add(offset);

    match flag {
        0 => false, // Not provided
        1 => true,  // Provided
        _ => panic!("Invalid presence flag: expected 0x00 or 0x01, got something else"),
    }
}

/// Determines whether a value is present by inspecting the byte at the given
/// offset.
///
/// Advances the offset by one byte. The presence byte must be `0x00` (absent)
/// or `0x01` (present); any other value causes a panic.
pub(crate) unsafe fn read_presence_flag(ptr: *const u8, offset: &mut usize) -> bool {
    *offset += 1;
    peek_presence_flag(ptr, *offset - 1)
}

/// Reads a little-endian u32 and advances the offset.
///
/// Safety: `ptr` must be valid for at least `*offset + 4` bytes.
#[inline]
pub(crate) unsafe fn read_u32(ptr: *const u8, offset: &mut usize) -> u32 {
    let ptr = ptr.add(*offset);
    *offset += 4;

    ptr.cast::<u32>().read_unaligned()
}

/// Reads a little-endian i32 and advances the offset.
///
/// Safety: `ptr` must be valid for at least `*offset + 4` bytes.
#[inline]
pub(crate) unsafe fn read_i32(ptr: *const u8, offset: &mut usize) -> i32 {
    let ptr = ptr.add(*offset);
    *offset += 4;

    ptr.cast::<i32>().read_unaligned()
}

/// Unregister an event handler without affecting EOSE subscriptions.
#[inline(never)]
pub(crate) fn remove_on_event_subscription(handle: i32) {
    let mut event_handlers = crate::SUBSCRIPTIONS_ON_EVENT.write();
    let length = event_handlers.len();
    let mut i = 0;

    while i < length {
        let entry = unsafe { event_handlers.get_unchecked(i) };
        if entry.0 == handle {
            unsafe { event_handlers.swap_remove_unchecked(i) };
        } else {
            i += 1;
        }
    }
}

/// Unregister an event handler without affecting on_event subscriptions.
#[inline(never)]
pub(crate) fn remove_on_eose_subscription(handle: i32) {
    let mut eose_handlers = crate::SUBSCRIPTIONS_ON_EOSE.write();
    let length = eose_handlers.len();
    let mut i = 0;

    while i < length {
        let entry = unsafe { eose_handlers.get_unchecked(i) };
        if entry.0 == handle {
            unsafe { eose_handlers.swap_remove_unchecked(i) };
        } else {
            i += 1;
        }
    }
}

/// Unregister a subscription from both event and EOSE handlers.
#[inline]
pub(crate) fn remove_subscription(handle: i32) {
    remove_on_event_subscription(handle);
    remove_on_eose_subscription(handle);
}

/// Look up the event callback for an active subscription.
#[inline(never)]
pub(crate) fn find_on_event_callback(handle: i32) -> Option<fn(Event, bool) -> bool> {
    let event_handlers = crate::SUBSCRIPTIONS_ON_EVENT.read();
    let length = event_handlers.len();
    let mut i = 0;

    while i < length {
        let entry = unsafe { event_handlers.get_unchecked(i) };
        if entry.0 == handle {
            return Some(entry.1.0);
        }
        i += 1;
    }

    None
}

/// Look up the EOSE callback for an active subscription.
#[inline(never)]
pub(crate) fn find_on_eose_callback(handle: i32) -> Option<fn() -> bool> {
    let eose_handlers = crate::SUBSCRIPTIONS_ON_EOSE.read();
    let length = eose_handlers.len();
    let mut i = 0;

    while i < length {
        let entry = unsafe { eose_handlers.get_unchecked(i) };
        if entry.0 == handle {
            return Some(entry.1);
        }
        i += 1;
    }

    None
}

/// Check whether a subscription is configured to auto-close on EOSE.
/// Returns false if the handle does not exist.
#[inline(never)]
pub(crate) fn is_close_on_eose(handle: i32) -> bool {
    let event_handlers = crate::SUBSCRIPTIONS_ON_EVENT.read();
    let length = event_handlers.len();
    let mut i = 0;

    while i < length {
        let entry = unsafe { event_handlers.get_unchecked(i) };
        if entry.0 == handle {
            return entry.1.1;
        }
        i += 1;
    }
    false
}
