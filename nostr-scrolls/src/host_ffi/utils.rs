// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::{Error, Result};

/// Ensures a length matches an expected value, failing with a detailed mismatch error.
#[inline(always)]
pub(crate) fn compare_size(size: usize, expected: usize) -> Result<()> {
    if size != expected {
        return Err(Error::SizeMismatch {
            expected,
            found: size,
        });
    }
    Ok(())
}

/// Validates that a byte length fits within a signed 32‑bit integer.
#[inline(always)]
pub(crate) fn ensure_size(size: usize) -> Result<()> {
    if size > i32::MAX as usize {
        return Err(Error::SizeOverflow);
    }
    Ok(())
}

/// Copies a slice from a raw pointer into an owned fixed-size array.
///
/// # Panics
/// Panics if `ptr` is null.
#[inline(always)]
pub(crate) fn read_slice_owned<const N: usize>(ptr: *const u8) -> [u8; N] {
    if ptr.is_null() {
        panic!("Invalid pointer by the host runtime")
    }

    let mut buffer = [0; N];
    buffer.copy_from_slice(unsafe { core::slice::from_raw_parts(ptr, 32) });
    buffer
}

/// Creates a byte slice reference from a raw pointer without copying.
///
/// # Panics
/// Panics if `ptr` is null.
#[inline(always)]
pub(crate) fn read_slice<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    if ptr.is_null() {
        panic!("Invalid pointer by the host runtime")
    }

    unsafe { core::slice::from_raw_parts(ptr, len) }
}

/// Reads a length-prefixed UTF-8 string from raw memory.
///
/// The length is stored as a 4-byte big-endian unsigned integer at the start,
/// followed by that many bytes of string data. The returned slice borrows from
/// the input pointer with lifetime `'a`.
///
/// # Safety
///
/// - `ptr` must be valid for reads of `4 + length` bytes.
/// - The length bytes must form a valid big-endian `u32`.
/// - The subsequent `length` bytes must be valid UTF-8.
#[inline(always)]
pub(crate) fn read_slice_string<'a>(ptr: *const u8) -> &'a str {
    unsafe {
        // Read the 4-byte length.
        let length = u32::from_be(ptr.cast::<u32>().read_unaligned());

        let str_ptr = ptr.add(4);
        let bytes = core::slice::from_raw_parts(str_ptr, length as usize);

        // In debug builds, verify UTF-8 validity to catch guest-side bugs early.
        debug_assert!(
            core::str::from_utf8(bytes).is_ok(),
            "Invalid UTF-8 in WASM memory"
        );

        core::str::from_utf8_unchecked(bytes)
    }
}
