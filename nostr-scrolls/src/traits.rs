// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::utils;

/// A trait to get the underlying host runtime handle of a value
pub trait IntoHandle {
    /// The raw handle value used by the host runtime.
    fn handle(&self) -> i32;
}

/// Trait for reading parameters from the memory. Each parameter type must
/// implement this trait to be used in `run` function parameters.
///
/// Blanket implementation provided for `Option<T> where T: ReadParam`.
pub trait ReadParam<'a>: Sized + 'a {
    /// Reads a value from memory at `ptr + *offset` and advances the offset.
    ///
    /// # Safety
    ///
    /// The caller must ensure that reading `Self` at the computed address is valid.
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self;
}

impl<'a, T> ReadParam<'a> for Option<T>
where
    T: ReadParam<'a>,
{
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self {
        if !utils::peek_presence_flag(ptr, *offset) {
            *offset += 1;
            return None;
        }

        Some(T::read_param(ptr, offset))
    }
}

// | Type         | Encoding                           | Size (if provided) |
// | ------------ | -----------------------------------| ------------------ |
// | `public_key` | 32 bytes                           | 32 bytes           |
// | `event`      | i32 handle                         | 4 bytes            |
// | `string`     | u32 length followed by UTF-8 bytes | 4 + len bytes      |
// | `number`     | i32                                | 4 bytes            |
// | `timestamp`  | Unix timestamp as u32              | 4 bytes            |
// | `relay`      | Relay URL, same encoding as string | 4 + len bytes      |

impl<'a> ReadParam<'a> for &'a str {
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self {
        if !utils::read_presence_flag(ptr, offset) {
            panic!("ReadParam(&str): Expected required parameter, but host provided 0x00");
        }

        let str_len = utils::read_u32(ptr, offset) as usize;
        let bytes = core::slice::from_raw_parts(ptr.add(*offset), str_len);
        *offset += str_len;

        debug_assert!(
            core::str::from_utf8(bytes).is_ok(),
            "ReadParam(&str): Invalid UTF-8 encountered in memory"
        );

        core::str::from_utf8_unchecked(bytes)
    }
}

impl<'a> ReadParam<'a> for i32 {
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self {
        if !utils::read_presence_flag(ptr, offset) {
            panic!("ReadParam(i32): Expected required parameter, but host provided 0x00");
        }

        utils::read_i32(ptr, offset)
    }
}

impl<'a> ReadParam<'a> for u32 {
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self {
        if !utils::read_presence_flag(ptr, offset) {
            panic!("ReadParam(u32): Expected required parameter, but host provided 0x00");
        }

        utils::read_u32(ptr, offset)
    }
}
