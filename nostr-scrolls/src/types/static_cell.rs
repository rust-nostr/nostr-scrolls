// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use core::cell::UnsafeCell;

/// Provides interior mutability for `static` items in single-threaded WebAssembly
/// without compile-time borrow checking.
///
/// # Safety
///
/// Dynamic borrow checking is the caller's responsibility.
///
/// # Example
///
/// ```rust,no_run
/// use nostr_scrolls::StaticCell;
///
/// static GLOBAL_COUNT: StaticCell<usize> = StaticCell::new(0);
///
/// fn increment() {
///     *GLOBAL_COUNT.get_mut() += 1;
/// }
/// ```
pub struct StaticCell<T>(UnsafeCell<T>);

// SAFETY: In single-threaded WASM, there are no threads to race with.
unsafe impl<T> Sync for StaticCell<T> {}

impl<T> StaticCell<T> {
    /// Wraps a value for use in static contexts.
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    /// Returns an immutable reference to the contained value.
    pub fn get(&self) -> &T {
        unsafe { &*self.0.get() }
    }

    /// Returns a mutable reference to the contained value.
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}
