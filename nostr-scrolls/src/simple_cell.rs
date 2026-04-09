// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use core::cell::{Cell, UnsafeCell};

/// A simple cell with mutable borrowing, similar to a `RefCell` but without
/// panic recovery.
pub(crate) struct SimpleCell<T> {
    value: UnsafeCell<T>,
    is_borrowed: Cell<bool>,
}

impl<T> SimpleCell<T> {
    /// Creates a new SimpleCell containing the given value.
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            is_borrowed: Cell::new(false),
        }
    }

    /// Borrows the value mutably. Panics if already borrowed.
    pub fn borrow(&self) -> SimpleGuard<'_, T> {
        if self.is_borrowed.get() {
            core::panic!("SimpleCell: already borrowed");
        }
        self.is_borrowed.set(true);
        SimpleGuard { cell: self }
    }
}

/// A guard that releases the borrow when dropped.
pub(crate) struct SimpleGuard<'a, T> {
    cell: &'a SimpleCell<T>,
}

impl<'a, T> core::ops::Deref for SimpleGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.cell.value.get() }
    }
}

impl<'a, T> core::ops::DerefMut for SimpleGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.cell.value.get() }
    }
}

impl<'a, T> Drop for SimpleGuard<'a, T> {
    fn drop(&mut self) {
        self.cell.is_borrowed.set(false);
    }
}
