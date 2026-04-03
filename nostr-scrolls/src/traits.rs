// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::utils;

/// A trait to get the underlying host runtime handle of a value
pub trait IntoHandle {
    /// The raw handle value used by the host runtime.
    fn handle(&self) -> i32;
}

/// Trait for reading parameters from the host-provided buffer. Each parameter
/// type must implement this trait to be used in `run` function parameters.
pub trait ReadParam<'a, 'b>: Sized {
    /// Read a parameter from the buffer at the current cursor position.
    /// Advances the cursor after reading.
    ///
    /// # Panics
    /// Panics if there are not enough bytes in the buffer.
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self;
}

// | Type         | Encoding                                                              | Size          |
// | ------------ | --------------------------------------------------------------------- | ------------- |
// | `public_key` | 32 bytes (should all be set to zero if the parameter is not provided) | 32 bytes      |
// | `event`      | i32 handle                                                            | 4 bytes       |
// | `string`     | u32_be length followed by UTF-8 bytes                                 | 4 + len bytes |
// | `number`     | i32                                                                   | 4 bytes       |
// | `timestamp`  | unix timestamp as u32_be                                              | 4 bytes       |
// | `relay`      | relay URL, same as string                                             | 4 + len bytes |

impl<'a, 'b> ReadParam<'a, 'b> for &'b str {
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self {
        if !utils::read_presence_flag(cursor, buffer) {
            panic!("ReadParam(&str): Expected required parameter, but host provided 0x00");
        }

        if buffer.len() < *cursor + 4 {
            panic!(
                "ReadParam(&str): Out of bounds reading length prefix at cursor {cursor} (buffer len: {})",
                buffer.len()
            );
        }

        let str_len = u32::from_be_bytes([
            buffer[*cursor],
            buffer[*cursor + 1],
            buffer[*cursor + 2],
            buffer[*cursor + 3],
        ]) as usize;

        *cursor += 4;

        if buffer.len() < *cursor + str_len {
            panic!(
                "ReadParam(&str): Out of bounds reading string payload (expected {str_len} bytes, remaining {})",
                buffer.len() - *cursor
            );
        }

        let bytes = &buffer[*cursor..*cursor + str_len];
        *cursor += str_len;

        debug_assert!(
            core::str::from_utf8(bytes).is_ok(),
            "ReadParam(&str): Invalid UTF-8 encountered in memory"
        );

        unsafe { core::str::from_utf8_unchecked(bytes) }
    }
}

impl<'a, 'b> ReadParam<'a, 'b> for Option<&'b str> {
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self {
        // Check the presence flag: if absent, consume 1 byte and return None;
        // otherwise delegate to the &str implementation to read the actual
        // string
        if !utils::peek_presence_flag(cursor, buffer) {
            *cursor += 1;
            return None;
        }

        Some(<&str as ReadParam>::read_param(cursor, buffer))
    }
}

impl<'a, 'b> ReadParam<'a, 'b> for i32 {
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self {
        if !utils::read_presence_flag(cursor, buffer) {
            panic!("ReadParam(i32): Expected required parameter, but host provided 0x00");
        }

        if buffer.len() < *cursor + 4 {
            panic!("ReadParam(i32): Out of bounds reading i32 at cursor {cursor}",);
        }

        let val = i32::from_le_bytes([
            buffer[*cursor],
            buffer[*cursor + 1],
            buffer[*cursor + 2],
            buffer[*cursor + 3],
        ]);

        *cursor += 4;
        val
    }
}

impl<'a, 'b> ReadParam<'a, 'b> for Option<i32> {
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self {
        // Check the presence flag: if absent, consume 1 byte and return None;
        // otherwise delegate to the i32 implementation to read the actual
        // number
        if !utils::peek_presence_flag(cursor, buffer) {
            *cursor += 1;
            return None;
        }

        Some(<i32 as ReadParam>::read_param(cursor, buffer))
    }
}

impl<'a, 'b> ReadParam<'a, 'b> for u32 {
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self {
        if !utils::read_presence_flag(cursor, buffer) {
            panic!("ReadParam(u32): Expected required parameter, but host provided 0x00");
        }

        if buffer.len() < *cursor + 4 {
            panic!("ReadParam(u32): Out of bounds reading u32 at cursor {cursor}",);
        }

        let val = u32::from_be_bytes([
            buffer[*cursor],
            buffer[*cursor + 1],
            buffer[*cursor + 2],
            buffer[*cursor + 3],
        ]);

        *cursor += 4;
        val
    }
}

impl<'a, 'b> ReadParam<'a, 'b> for Option<u32> {
    fn read_param(cursor: &'a mut usize, buffer: &'b [u8]) -> Self {
        // Check the presence flag: if absent, consume 1 byte and return None;
        // otherwise delegate to the u32 implementation to read the actual
        // number
        if !utils::peek_presence_flag(cursor, buffer) {
            *cursor += 1;
            return None;
        }

        Some(<u32 as ReadParam>::read_param(cursor, buffer))
    }
}
