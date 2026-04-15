// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use core::{
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// A mutable memory location with dynamic borrow checking, explicitly designed
/// for `static` contexts in single-threaded WebAssembly.
///
/// This provides interior mutability similar to [`RefCell`], but safely
/// implements `Sync`. In standard Rust, `static` items require [`Sync`],
/// which prevents the use of [`RefCell`] for global mutable state. Because
/// [`nostr_scrolls`] strictly targets single-threaded `wasm32`, implementing
/// [`Sync`] for this type is sound, allowing you to bypass the thread-safety
/// checks without using heavy synchronization primitives like `Mutex` or
/// `RwLock`.
///
/// # Panics
///
/// Panics at runtime if the borrow rules are violated (e.g., attempting a
/// mutable borrow while shared borrows exist).
///
/// # Example
///
/// ```rust,no_run
/// use nostr_scrolls::StaticCell;
///
/// static GLOBAL_COUNT: StaticCell<usize> = StaticCell::new(0);
///
/// fn increment() {
///     *GLOBAL_COUNT.borrow_mut() += 1;
/// }
/// ```
///
/// [`RefCell`]: core::cell::RefCell
/// [`Sync`]: core::marker::Sync
/// [`nostr_scrolls`]: crate
pub struct StaticCell<T> {
    value: UnsafeCell<T>,
    // 0 = unborrowed, > 0 and < u16::MAX = shared borrows, u16::MAX = mutable borrow
    borrow_state: UnsafeCell<u16>,
}

// SAFETY: In single-threaded WASM, there are no threads to race with.
unsafe impl<T> Sync for StaticCell<T> {}

impl<T> StaticCell<T> {
    /// Create a new static variable.
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            borrow_state: UnsafeCell::new(0),
        }
    }

    /// Acquires an immutable guard.
    ///
    /// # Panics
    /// Panics if a mutable borrow is currently active.
    pub fn borrow(&self) -> StaticCellRef<'_, T> {
        unsafe {
            let state = &mut *self.borrow_state.get();

            if *state == u16::MAX {
                panic!("StaticCell already mutably borrowed");
            }

            if *state == u16::MAX - 1 {
                panic!("StaticCell shared borrow overflow");
            }

            // Increment shared borrow count
            *state += 1;

            StaticCellRef {
                value: self.value.get(),
                state: self.borrow_state.get(),
                _marker: PhantomData,
            }
        }
    }

    /// Acquires a mutable guard.
    ///
    /// # Panics
    /// Panics if any immutable or mutable borrows are currently active.
    pub fn borrow_mut(&self) -> StaticCellMut<'_, T> {
        unsafe {
            let state = &mut *self.borrow_state.get();

            if *state > 0 && *state != u16::MAX {
                panic!("StaticCell already borrowed immutably");
            } else if *state == u16::MAX {
                panic!("StaticCell already mutably borrowed");
            }

            // Mark as exclusively borrowed
            *state = u16::MAX;

            StaticCellMut {
                value: self.value.get(),
                state: self.borrow_state.get(),
                _marker: PhantomData,
            }
        }
    }
}

/// RAII guard for an immutable borrow.
pub struct StaticCellRef<'a, T> {
    value: *mut T,
    state: *mut u16,
    // Ties the guard's lifetime to the StaticCell
    _marker: PhantomData<&'a T>,
}

impl<T> Deref for StaticCellRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The borrow_state guarantees no mutable references exist, and
        // the guard's lifetime guarantees the StaticCell is still alive.
        unsafe { &*self.value }
    }
}

impl<T> Drop for StaticCellRef<'_, T> {
    fn drop(&mut self) {
        // SAFETY: We know we were borrowed immutably, so state is > 0.
        unsafe {
            *self.state -= 1;
        }
    }
}

/// RAII guard for a mutable borrow.
pub struct StaticCellMut<'a, T> {
    value: *mut T,
    state: *mut u16,
    // Uses &mut to prevent creating multiple mutable guards
    _marker: PhantomData<&'a mut T>,
}

impl<T> Deref for StaticCellMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

impl<T> DerefMut for StaticCellMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value }
    }
}

impl<T> Drop for StaticCellMut<'_, T> {
    fn drop(&mut self) {
        // SAFETY: We know we held the mutable borrow, so state is exactly -1.
        unsafe {
            *self.state = 0;
        }
    }
}
