//! A simple program demonstrating the use of the `from` attribute. It receives
//! a target event and returns notes from the same author within the specified time
//! window.
//!
//! Note: We panic inside `From`, so `try_from` is unnecessary, simply panic instead
//! of returning an error.
//!
//! Program parameters:
//! ```
//! [
//!   ["param", "Target Event", "", "event", "required"],
//!   ["param", "Time Window", "Time window in days", "number", "required"],
//! ]
//! ```

#![no_std]
#![no_main]

extern crate alloc;

use nostr_scrolls::{Event, Filter, cb, display};

struct Days(usize);

impl From<i32> for Days {
    fn from(value: i32) -> Self {
        if value <= 0 {
            panic!("Days must be greater than 0");
        }

        // Maximum: 49710 days fits within i32::MAX seconds
        if value > 49710 {
            panic!("The days exceeds maximum supported value",);
        }

        Self(value as usize)
    }
}

impl Days {
    fn as_secs(&self) -> usize {
        self.0 * 24 * 60 * 60
    }
}

#[allow(unused_must_use)]
#[nostr_scrolls::main]
fn run(event: Event, #[from(i32)] day_window: Days) {
    let mut filter = Filter::new();
    filter.kind(1);
    filter.author(&event.pubkey());
    filter.since(event.created_at() - day_window.as_secs());
    filter.until(event.created_at() + day_window.as_secs());
    filter.subscribe().on_event(cb!(|e| display(&e)));
}

// Expanded code will looks like:
//
// pub unsafe extern "C" fn run(ptr: *const u8) {
//     let mut offset = 0usize;
//     if ptr.is_null() {
//         {
//             ::core::panicking::panic_fmt(format_args!(
//                 "null pointer passed as a parameters pointer"
//             ));
//         };
//     }
//     let event: Event = <Event as nostr_scrolls::ReadParam>::read_param(ptr, &mut offset);
//     let day_window: Days = <Days as core::convert::From<i32>>::from(
//         <i32 as nostr_scrolls::ReadParam>::read_param(ptr, &mut offset),
//     );
//     {
//         // The above `run` block
//     }
// }
