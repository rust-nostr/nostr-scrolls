// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::ReadParam;

/// A number that can't be negative
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PositiveNumber(i32);

impl ReadParam<'_> for PositiveNumber {
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self {
        let num = <i32 as ReadParam>::read_param(ptr, offset);
        if num < 0 {
            panic!("ReadParam(PositiveNumber): expected a positive number, found a negative one");
        }
        Self(num)
    }
}

impl From<PositiveNumber> for u32 {
    fn from(value: PositiveNumber) -> Self {
        value.0 as u32
    }
}

impl From<PositiveNumber> for usize {
    fn from(value: PositiveNumber) -> Self {
        value.0 as usize
    }
}

/// A number that can't be positive
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NegativeNumber(i32);

impl ReadParam<'_> for NegativeNumber {
    unsafe fn read_param(ptr: *const u8, offset: &mut usize) -> Self {
        let num = <i32 as ReadParam>::read_param(ptr, offset);
        if num > 0 {
            panic!("ReadParam(NegativeNumber): expected a negative number, found a positive one");
        }
        Self(num)
    }
}

impl From<NegativeNumber> for i32 {
    fn from(value: NegativeNumber) -> Self {
        value.0
    }
}

impl From<NegativeNumber> for isize {
    fn from(value: NegativeNumber) -> Self {
        value.0 as isize
    }
}
