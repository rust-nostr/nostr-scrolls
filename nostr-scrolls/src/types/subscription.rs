// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use crate::{Event, host_ffi::drop as ffi_drop};

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
    /// The boolean flag indicates EOSE status: `true` if the event arrived
    /// after the End of Stored Events marker, `false` if before.
    ///
    /// Returning `true` from the handler terminates the subscription early.
    ///
    /// Note: Calling this function multiple time will not attach multiple
    /// handlers for the subscription, only last handler will be attached
    pub fn on_event(&self, handler: fn(Event, bool) -> bool) {
        let mut handlers = crate::SUBSCRIPTIONS_ON_EVENT.write();

        handlers.retain(|(handle, _)| handle != &self.handle);
        handlers
            .push((self.handle, (handler, self.close_on_eose)))
            .expect("The handlers is full");
    }

    /// Attach a callback invoked when the end-of-stored-events marker is
    /// received.
    ///
    /// Return `true` to close the subscription. Ignored if the subscription
    /// was already configured to close on EOSE via [`Filter::close_on_eose`].
    ///
    /// Note: Calling this function multiple time will not attach multiple
    /// handlers for the subscription, only last handler will be attached
    ///
    /// [`Filter::close_on_eose`]: crate::Filter::close_on_eose
    pub fn on_eose(&self, handler: fn() -> bool) {
        let mut handlers = crate::SUBSCRIPTIONS_ON_EOSE.write();

        handlers.retain(|(handle, _)| handle != &self.handle);
        handlers
            .push((self.handle, handler))
            .expect("The handlers is full");
    }

    /// Cancel the subscription. Terminating event delivery
    pub fn cancel(self) {
        let mut event_handlers = crate::SUBSCRIPTIONS_ON_EVENT.write();
        let mut eose_handlers = crate::SUBSCRIPTIONS_ON_EOSE.write();

        event_handlers.retain(|(handle, _)| handle != &self.handle);
        eose_handlers.retain(|(handle, _)| handle != &self.handle);
        unsafe { ffi_drop(self.handle) };
    }
}
