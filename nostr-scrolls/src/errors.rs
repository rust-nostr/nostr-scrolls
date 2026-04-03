// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use core::fmt;

/// Internal result type.
pub(crate) type Result<T> = core::result::Result<T, Error>;

/// nostr-scrolls errors
#[derive(fmt::Debug)]
pub enum Error {
    /// Value exceeds the maximum size representable by [`i32::MAX`].
    SizeOverflow,

    /// The actual value size doesn't match the expected size.
    SizeMismatch {
        /// Expected length of the value.
        expected: usize,
        /// Actual length encountered.
        found: usize,
    },

    /// Invalid tag, it must be an ASCII alphabetic character
    InvalidTag,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SizeOverflow => {
                write!(f, "size exceeds maximum allowed value of {}", i32::MAX)
            }
            Self::SizeMismatch { expected, found } => {
                write!(f, "expected value of size {expected}, got {found}")
            }
            Self::InvalidTag => write!(f, "invalid tag, should be an ASCII alphabetic character"),
        }
    }
}

impl core::error::Error for Error {}
