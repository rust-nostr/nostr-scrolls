// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![warn(clippy::large_futures)]
#![doc = include_str!("../../README.md")]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("This crate support wasm32 only");

mod errors;
mod host_ffi;
mod traits;
mod types;
mod utils;

use heapless::Vec;
use spin::RwLock;

pub use self::errors::*;
pub use self::host_ffi::safe_wrapper::{display, log};
pub use self::traits::*;
pub use self::types::*;
pub use nostr_scrolls_macros::main;

/// Maps subscription handles to their event handlers and whether to close on
/// EOSE.
#[allow(clippy::type_complexity)]
pub(crate) static SUBSCRIPTIONS_ON_EVENT: RwLock<Vec<(i32, (fn(Event, bool) -> bool, bool)), 128>> =
    RwLock::new(Vec::new());

/// Maps subscription handles to their EOSE handlers
#[allow(clippy::type_complexity)]
pub(crate) static SUBSCRIPTIONS_ON_EOSE: RwLock<Vec<(i32, fn() -> bool), 128>> =
    RwLock::new(Vec::new());

static mut BUMP_PTR: usize = 0;

/// Allocates unmanaged memory for the host with no alignment requirements.
#[unsafe(no_mangle)]
#[doc(hidden)]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    unsafe {
        if BUMP_PTR == 0 {
            // start just past the stack (rough heuristic, or use a linker symbol)
            BUMP_PTR = core::arch::wasm32::memory_size(0) * 65536;
            core::arch::wasm32::memory_grow(0, 1);
        }
        let ptr = BUMP_PTR;
        BUMP_PTR += size;
        ptr as *mut u8
    }
}

// If the WASM module ever calls `nostr.subscribe` it must also export a
// function named `on_event` that will be called with every received event from
// any subscription. `sub` will be the subscription handle, `event` will be the
// event handle, `eosed` will be `0` if the event was received before `EOSE`, `1`
// otherwise.
/// Dispatches events to registered subscription callbacks, automatically
/// cleaning up closed subscriptions.
///
/// This is the global FFI entry point invoked by the native library when
/// events arrive. If a callback returns `true`, the subscription is dropped and
/// removed from the handler map.
#[unsafe(no_mangle)]
#[doc(hidden)]
pub unsafe extern "C" fn on_event(sub_handle: i32, event_handle: i32, eosed: i32) {
    let on_event_handlers = SUBSCRIPTIONS_ON_EVENT.read();

    if let Some((_, (callback, _))) = on_event_handlers
        .iter()
        .find(|(handle, _)| handle == &sub_handle)
    {
        let close_sub = (callback)(
            Event {
                handle: event_handle,
            },
            eosed != 0,
        );

        if close_sub {
            core::mem::drop(on_event_handlers);
            Subscription::from_handle(sub_handle).cancel();
        }
    }
}

// Likewise, this will be called by the host whenever a subscription sends an
// `EOSE`.
/// Dispatches EOSE to registered subscription callbacks, automatically cleaning
/// up closed subscriptions.
///
/// This is the global FFI entry point invoked by the native library when
/// EOSE arrive. If a callback returns `true`, the subscription is dropped and
/// removed from the handler map.
#[unsafe(no_mangle)]
#[doc(hidden)]
pub unsafe extern "C" fn on_eose(sub_handle: i32) {
    let eose_handlers = SUBSCRIPTIONS_ON_EOSE.read();
    let event_handlers = SUBSCRIPTIONS_ON_EVENT.read();

    // Try to find a custom EOSE handler for this subscription
    if let Some((_, callback)) = eose_handlers
        .iter()
        .find(|(handle, _)| handle == &sub_handle)
    {
        // Execute the user's custom EOSE callback; true indicates subscription
        // should close
        let close_sub = (callback)();

        // Check if this subscription is configured to auto-close on EOSE
        let is_close_on_eose = matches!(
            event_handlers
                .iter()
                .find(|(handle, _)| handle == &sub_handle),
            Some((_, (_, true)))
        );

        // Remove subscription from both handler maps if either:
        // - The custom EOSE handler returned true (wants to close), OR
        // - The subscription is configured to auto-close on EOSE
        if close_sub || is_close_on_eose {
            core::mem::drop(eose_handlers);
            core::mem::drop(event_handlers);

            Subscription::from_handle(sub_handle).cancel();
        }
    } else {
        // No custom EOSE handler exists for this subscription.
        // Check if it's in the event handlers and configured to auto-close on EOSE.
        // If so, remove it from event handlers to prevent further event processing.
        if let Some((_, (_, true))) = event_handlers
            .iter()
            .find(|(handle, _)| handle == &sub_handle)
        {
            core::mem::drop(event_handlers);

            let mut event_handlers = SUBSCRIPTIONS_ON_EVENT.write();
            event_handlers.retain(|(handle, _)| handle != &sub_handle);
        }
    }
}
