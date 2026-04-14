// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use ::alloc::boxed::Box;

use crate::Event;

/// Callback variants for handling Nostr events with varying signatures.
pub enum EventCallback {
    /// Processes an event with no return value.
    Arg(Box<dyn FnMut(Event) + 'static>),
    /// Processes an event and returns whether to close the subscription.
    ArgReturn(Box<dyn FnMut(Event) -> bool + 'static>),
    /// Processes an event and EOSE status with no return value.
    Args(Box<dyn FnMut(Event, bool) + 'static>),
    /// Processes an event and EOSE status, returning whether to close the
    /// subscription.
    ArgsReturn(Box<dyn FnMut(Event, bool) -> bool + 'static>),
}

impl EventCallback {
    /// Dispatches an event to the wrapped callback
    pub fn call(&mut self, event: Event, eosed: bool) -> bool {
        match self {
            Self::Arg(fn_mut) => fn_mut(event),
            Self::ArgReturn(fn_mut) => return fn_mut(event),
            Self::Args(fn_mut) => fn_mut(event, eosed),
            Self::ArgsReturn(fn_mut) => return fn_mut(event, eosed),
        }
        false
    }
}

/// Callback variants for handling End of Stored Events (EOSE) messages.
pub enum EoseCallback {
    /// Runs on EOSE with no return value, keeping the subscription open.
    NoReturn(Box<dyn FnMut() + 'static>),
    /// Runs on EOSE and returns whether to close the subscription.
    Return(Box<dyn FnMut() -> bool + 'static>),
}

impl EoseCallback {
    /// Call EOSE callback
    pub fn call(&mut self) -> bool {
        match self {
            Self::NoReturn(fn_mut) => fn_mut(),
            Self::Return(fn_mut) => return fn_mut(),
        }
        false
    }
}

/// Creates a callback that keeps the subscription open
///
/// Accepts closures with 0, 1, or 2 parameters:
/// - `||` — EOSE callback with no arguments
/// - `|event|` — Event callback with the event only
/// - `|event, eosed|` — Event callback with event and EOSE flag
///
/// ## Examples
///
/// ```rust,ignore
/// use nostr_scrolls::{Filter, cb, cb_ret};
///
/// let mut filter = Filter::new()
///     .kind(1)
///     .limit(10);
///
/// let sub = filter.subscribe();
/// sub.on_event(cb!(|event| /* ... */)); // A callback that only take an event
/// sub.on_event(cb!(|event, eosed| /* ... */)); // A callback that take both
/// sub.on_eose(cb!(|| /* ... */)); // A EOSE callback that return nothing
/// ```
#[macro_export]
macro_rules! cb {
    (|| $body:expr) => {
        $crate::EoseCallback::NoReturn(::alloc::boxed::Box::new(|| $body))
    };
    (|$a:ident| $body:expr) => {
        $crate::EventCallback::Arg(::alloc::boxed::Box::new(move |$a| $body))
    };
    (|$a:ident, $b:ident| $body:expr) => {
        $crate::EventCallback::Args(::alloc::boxed::Box::new(|$a, $b| $body))
    };
}

/// Creates a callback that may close the subscription based on its return value.
///
/// The closure must return `bool`: `true` to keep the subscription open, `false` to close it.
/// Accepts closures with 0, 1, or 2 parameters:
/// - `||` — EOSE callback with no arguments
/// - `|event|` — Event callback with the event only
/// - `|event, eosed|` — Event callback with event and EOSE flag
///
/// ## Examples
///
/// ```rust,ignore
/// use nostr_scrolls::{Filter, cb, cb_ret};
///
/// let mut filter = Filter::new()
///     .kind(1)
///     .limit(10);
///
/// let sub = filter.subscribe();
/// sub.on_event(cb_ret!(|event| /* bool */)); // A callback that only take an
///                                            // event and return whenever to
///                                            // close the sub or not
/// sub.on_event(cb_ret!(|event, eosed| /* bool */)); // A callback that take both
/// sub.on_eose(cb_ret!(|| /* bool */)); // A EOSE callback that return
/// ```
#[macro_export]
macro_rules! cb_ret {
    (|| $body:expr) => {
        $crate::EoseCallback::Return(::alloc::boxed::Box::new(|| $body))
    };
    (|$a:ident| $body:expr) => {
        $crate::EventCallback::ArgReturn(::alloc::boxed::Box::new(|$a| $body))
    };
    (|$a:ident, $b:ident| $body:expr) => {
        $crate::EventCallback::ArgsReturn(::alloc::boxed::Box::new(|$a, $b| $body))
    };
}
