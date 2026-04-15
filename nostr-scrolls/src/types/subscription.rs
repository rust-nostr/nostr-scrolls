// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::{EoseCallback, EventCallback, host_ffi::drop as ffi_drop, utils};

/// Nostr scrolls subscription
pub struct Subscription {
    /// The subscription handle
    pub(crate) handle: i32,
    pub(crate) close_on_eose: bool,
}

impl Subscription {
    /// Create a subscription from a handler
    pub(crate) fn from_handle(handle: i32) -> Self {
        Self {
            handle,
            close_on_eose: false,
        }
    }

    /// Register a handler that is invoked for every event received on this
    /// subscription.
    ///
    /// You can use [`cb!`] or [`cb_ret!`] macros to make createing a callback easier
    ///
    /// Note: Calling this function multiple time will not attach multiple
    /// handlers for the subscription, only last handler will be attached
    ///
    /// [`cb!`]: crate::cb
    /// [`cb_ret!`]: crate::cb_ret
    pub fn on_event(&self, handler: EventCallback) {
        utils::remove_on_event_subscription(self.handle);

        if crate::SUBSCRIPTIONS_ON_EVENT
            .borrow_mut()
            .push((self.handle, (self.close_on_eose, handler)))
            .is_err()
        {
            #[cfg(not(feature = "debug-strings"))]
            panic!("Faild to register a new `on_event` handler. The handlers is full");
            #[cfg(feature = "debug-strings")]
            panic!(
                "Faild to register a new `on_event` handler for `{}` subscription. The handlers is full",
                self.handle
            );
        }
    }

    /// Attach a callback invoked when the end-of-stored-events marker is
    /// received.
    ///
    /// You can use [`cb!`] or [`cb_ret!`] macros to make createing a callback easier
    ///
    /// Note: Calling this function multiple time will not attach multiple
    /// handlers for the subscription, only last handler will be attached
    ///
    /// [`Filter::close_on_eose`]: crate::Filter::close_on_eose
    /// [`cb!`]: crate::cb
    /// [`cb_ret!`]: crate::cb_ret
    pub fn on_eose(&self, handler: EoseCallback) {
        utils::remove_on_eose_subscription(self.handle);

        if crate::SUBSCRIPTIONS_ON_EOSE
            .borrow_mut()
            .push((self.handle, handler))
            .is_err()
        {
            #[cfg(not(feature = "debug-strings"))]
            panic!("Faild to register a new `on_eose` handler. The handlers is full");
            #[cfg(feature = "debug-strings")]
            panic!(
                "Faild to register a new `on_eose` handler for `{}` subscription. The handlers is full",
                self.handle
            );
        }
    }

    /// Cancel the subscription. Terminating event delivery
    #[inline(never)]
    pub fn cancel(self) {
        utils::remove_subscription(self.handle);
        unsafe { ffi_drop(self.handle) };
    }
}
