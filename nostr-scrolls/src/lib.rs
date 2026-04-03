// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

#![no_std]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![warn(clippy::large_futures)]
#![doc = include_str!("../../README.md")]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod errors;
mod host_ffi;
mod traits;
mod types;
mod utils;

extern crate alloc as sys_alloc;

use core::alloc::Layout;

use hashbrown::HashMap;
use spin::{Lazy, Mutex};
use sys_alloc::boxed::Box;

pub use self::errors::*;
pub use self::host_ffi::safe_wrapper::{display, drop, log};
pub use self::traits::*;
pub use self::types::*;
pub use nostr_scrolls_macros::main;

type SubHashMap<V> = Lazy<Mutex<HashMap<i32, V>>>;

/// Maps subscription handles to their event handlers and whether to close on
/// EOSE.
///
/// The boolean flag indicates whether the subscription should auto-close when
/// an EOSE message is received.
#[allow(clippy::type_complexity)]
pub(crate) static SUBSCRIPTIONS_ON_EVENT: SubHashMap<(
    Box<dyn FnMut(Event, bool) -> bool + Send>,
    bool,
)> = Lazy::new(|| Mutex::new(HashMap::new()));

/// Maps subscription handles to their EOSE handlers
pub(crate) static SUBSCRIPTIONS_ON_EOSE: SubHashMap<Box<dyn FnMut() -> bool + Send>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Allocates unmanaged memory for the host with no alignment requirements.
#[unsafe(no_mangle)]
#[doc(hidden)]
pub unsafe extern "C" fn alloc(size: i32) -> i32 {
    if size <= 0 {
        return 0;
    }

    // Alignment of 1 - no alignment requirements
    let layout = Layout::from_size_align(size as usize, 1).unwrap();
    let ptr = unsafe { sys_alloc::alloc::alloc(layout) };

    if ptr.is_null() { 0 } else { ptr as i32 }
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
    let mut eose_handlers = SUBSCRIPTIONS_ON_EOSE.lock();
    let mut on_event_handlers = SUBSCRIPTIONS_ON_EVENT.lock();

    if let Some((callback, _)) = on_event_handlers.get_mut(&sub_handle) {
        let close_sub = (callback)(
            Event {
                handle: event_handle,
            },
            eosed != 0,
        );

        if close_sub {
            crate::drop(Subscription::from_handle(sub_handle));
            on_event_handlers.retain(|handle, _| handle != &sub_handle);
            eose_handlers.retain(|handle, _| handle != &sub_handle);
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
    let mut eose_handlers = SUBSCRIPTIONS_ON_EOSE.lock();
    let mut event_handlers = SUBSCRIPTIONS_ON_EVENT.lock();

    // Try to find a custom EOSE handler for this subscription
    if let Some(callback) = eose_handlers.get_mut(&sub_handle) {
        // Execute the user's custom EOSE callback; true indicates subscription
        // should close
        let close_sub = (callback)();

        // Check if this subscription is configured to auto-close on EOSE
        let is_close_on_eose = matches!(event_handlers.get(&sub_handle), Some((_, true)));

        // Remove subscription from both handler maps if either:
        // - The custom EOSE handler returned true (wants to close), OR
        // - The subscription is configured to auto-close on EOSE
        if close_sub || is_close_on_eose {
            eose_handlers.retain(|handle, _| handle != &sub_handle);
            event_handlers.retain(|handle, _| handle != &sub_handle);
        }

        // The host automatically drops subscriptions that are configured to close on EOSE.
        // To avoid double-close/double-drop, we only manually close here if:
        // - The custom handler requested close, AND
        // - The host will NOT auto-close
        if close_sub && !is_close_on_eose {
            crate::drop(Subscription::from_handle(sub_handle));
        }
    } else {
        // No custom EOSE handler exists for this subscription.
        // Check if it's in the event handlers and configured to auto-close on EOSE.
        // If so, remove it from event handlers to prevent further event processing.
        if let Some((_, true)) = event_handlers.get(&sub_handle) {
            event_handlers.retain(|handle, _| handle != &sub_handle);
        }
    }
}
