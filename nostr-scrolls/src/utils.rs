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
