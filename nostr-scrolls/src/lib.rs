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

#[cfg(target_feature = "atomics")]
compile_error!("This crate does not support multi-threaded WebAssembly (wasm with atomics)");

mod allocator;
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
    if let Some(callback) = utils::find_on_event_callback(sub_handle) {
        let close_sub = (callback)(
            Event {
                handle: event_handle,
            },
            eosed != 0,
        );

        if close_sub {
            utils::remove_on_event_subscription(sub_handle);
            host_ffi::drop(sub_handle);
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
    // Try to find a custom EOSE handler for this subscription
    if let Some(callback) = utils::find_on_eose_callback(sub_handle) {
        // Execute the user's custom EOSE callback; true indicates subscription
        // should close
        let close_sub = (callback)();

        // Check if this subscription is configured to auto-close on EOSE
        let is_close_on_eose = utils::is_close_on_eose(sub_handle);

        // Remove subscription from both handler maps if either:
        // - The custom EOSE handler returned true (wants to close), OR
        // - The subscription is configured to auto-close on EOSE
        if close_sub || is_close_on_eose {
            Subscription::from_handle(sub_handle).cancel();
        }
    } else {
        // No custom EOSE handler exists for this subscription.
        // Check if it's in the event handlers and configured to auto-close on EOSE.
        // If so, remove it from event handlers to prevent further event processing.
        if utils::is_close_on_eose(sub_handle) {
            utils::remove_on_event_subscription(sub_handle);
            host_ffi::drop(sub_handle);
        }
    }
}
