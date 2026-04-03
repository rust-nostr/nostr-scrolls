// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

/// Determines whether a value is present by inspecting the next byte in the
/// buffer.
///
/// The presence byte must be `0` (absent) or `1` (present); any other value
/// causes a panic. Panics if the buffer is exhausted before the byte can be
/// read.
#[inline]
pub(crate) fn peek_presence_flag(cursor: &usize, buffer: &[u8]) -> bool {
    if buffer.len() < *cursor + 1 {
        panic!("Out of bounds reading presence flag");
    }

    let flag = buffer[*cursor];

    match flag {
        0 => false, // Not provided
        1 => true,  // Provided
        _ => panic!("Invalid presence flag: expected 0x00 or 0x01, got {flag:#04x}"),
    }
}

/// Determines whether a value is present by inspecting the next byte in the
/// buffer.
///
/// Advances the cursor by one byte. The presence byte must be `0` (absent)
/// or `1` (present); any other value causes a panic. Panics if the buffer is
/// exhausted before the byte can be read.
#[inline]
pub(crate) fn read_presence_flag(cursor: &mut usize, buffer: &[u8]) -> bool {
    let val = peek_presence_flag(cursor, buffer);
    *cursor += 1;
    val
}
