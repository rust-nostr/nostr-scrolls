//! Recommends events based on what likers of a target event also liked.
//!
//! Analyzes like patterns within a configurable time window (default: +/-7 days
//! from target event) to suggest related content that may interest the same
//! audience.
//!
//! Program parameters:
//! ```
//! [
//!   ["param", "Target Event", "Event to get related events to it", "event", "required"],
//!   ["param", "Time Window", "The time window in days", "number", ""],
//!   ["param", "Limit", "Limit the number of the results", "number", ""],
//! ]
//! ```

// Since you are looking at the example this means you are interested, I'm right?
//
// Okay. This example can be better, but I stopped to not make it complicated,
// but you can return shared likes and reposts and display the actual liked
// events not the reaction or the repost. Make sure to display root events
// only and not replies (hint: Check NIP-10). Do not forget to give me feedback
// (Awiteb) I like to talk with people

#![no_std]
#![no_main]

extern crate alloc;

use core::cell::UnsafeCell;

use nostr_scrolls::{Event, Filter, UnsafeSync, cb, display};

const REPOST_KIND: u16 = 6;
const REACTION_KIND: u16 = 7;
const SEVEN_DAYS: i32 = 60 * 60 * 24 * 7;

static RELATED_FILTER: UnsafeSync<UnsafeCell<Option<Filter>>> = UnsafeSync(UnsafeCell::new(None));

/// Build a filter capturing reposts and reactions near a target timestamp.
fn related_filter(target_ts: usize, time_window: Option<i32>, limit: Option<i32>) -> Filter {
    let time_window = time_window
        .and_then(|days| days.checked_mul(24 * 60 * 60))
        .unwrap_or(SEVEN_DAYS) as usize;

    let mut filter = Filter::new()
        .kind(REPOST_KIND)
        .kind(REACTION_KIND)
        .since(target_ts - time_window)
        .until(target_ts + time_window)
        .close_on_eose();

    if let Some(limit) = limit {
        filter = filter.limit(limit as usize);
    }

    filter
}

#[nostr_scrolls::main]
fn run(event: Event, time_window: Option<i32>, limit: Option<i32>) {
    nostr_scrolls::log("resonate: loading target event");

    if time_window.is_some_and(|days| days <= 0) {
        panic!("time window must be positive");
    }

    if limit.is_some_and(|l| l <= 0) {
        panic!("limit must be positive");
    }

    unsafe { *RELATED_FILTER.get() = Some(related_filter(event.created_at(), time_window, limit)) };

    // Find likes and reposts for this event
    let like_re_sub = Filter::new()
        .kind(REPOST_KIND)
        .kind(REACTION_KIND)
        .tag('e', event.id_hex())
        .tag('p', event.pubkey_hex())
        .close_on_eose()
        .subscribe();

    // Add authors of likes/reposts to filter
    like_re_sub.on_event(cb!(|event| unsafe {
        let filter = (*RELATED_FILTER.get()).take().unwrap_unchecked();
        (*RELATED_FILTER.get()) = Some(filter.author(&event.pubkey()));
    }));

    // Subscribe and display events liked by those authors
    like_re_sub.on_eose(cb!(|| unsafe {
        let filter = (&mut *RELATED_FILTER.get()).take().unwrap_unchecked();
        filter.subscribe().on_event(cb!(|e| display(&e)));
    }));
}
