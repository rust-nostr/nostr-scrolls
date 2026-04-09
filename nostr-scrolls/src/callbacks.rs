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

/// Creates an event or EOSE callback that keeps the subscription open.
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

/// Creates an event or EOSE callback that may close the subscription based on
/// return value.
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
