// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

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

/// Locates a subscription handle in the global `on_event` handler list.
#[inline(never)]
pub(crate) fn on_event_position(handle: i32) -> Option<usize> {
    let event_handlers = crate::SUBSCRIPTIONS_ON_EVENT.get();
    let length = event_handlers.len();
    let mut i = 0;

    while i < length {
        let entry = unsafe { event_handlers.get_unchecked(i) };
        if entry.0 == handle {
            return Some(i);
        }
        i += 1;
    }

    None
}

/// Locates a subscription handle in the global `EOSE` handler list.
#[inline(never)]
pub(crate) fn on_eose_position(handle: i32) -> Option<usize> {
    let eose_handlers = crate::SUBSCRIPTIONS_ON_EOSE.get();
    let length = eose_handlers.len();
    let mut i = 0;

    while i < length {
        let entry = unsafe { eose_handlers.get_unchecked(i) };
        if entry.0 == handle {
            return Some(i);
        }
        i += 1;
    }

    None
}

/// Unregister an event handler without affecting EOSE subscriptions.
#[inline(never)]
pub(crate) fn remove_on_event_subscription(handle: i32) {
    let Some(position) = on_event_position(handle) else {
        return;
    };

    unsafe {
        crate::SUBSCRIPTIONS_ON_EVENT
            .get_mut()
            .swap_remove_unchecked(position);
    }
}

/// Unregister an event handler without affecting on_event subscriptions.
#[inline(never)]
pub(crate) fn remove_on_eose_subscription(handle: i32) {
    let Some(position) = on_eose_position(handle) else {
        return;
    };

    unsafe {
        crate::SUBSCRIPTIONS_ON_EOSE
            .get_mut()
            .swap_remove_unchecked(position);
    }
}

/// Unregister a subscription from both event and EOSE handlers.
#[inline]
pub(crate) fn remove_subscription(handle: i32) {
    remove_on_event_subscription(handle);
    remove_on_eose_subscription(handle);
}

/// Check whether a subscription is configured to auto-close on EOSE.
/// Returns false if the handle does not exist.
#[inline(never)]
pub(crate) fn is_close_on_eose(handle: i32) -> bool {
    let Some(position) = on_event_position(handle) else {
        return false;
    };

    unsafe {
        crate::SUBSCRIPTIONS_ON_EVENT
            .get()
            .get_unchecked(position)
            .1
            .0
    }
}
