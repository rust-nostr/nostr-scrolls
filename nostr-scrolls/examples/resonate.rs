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

use nostr_scrolls::{Event, Filter, PositiveNumber, StaticCell, StaticFilter, cb, display};

const REPOST_KIND: u16 = 6;
const REACTION_KIND: u16 = 7;
const SEVEN_DAYS: usize = 60 * 60 * 24 * 7;

static RELATED_FILTER: StaticFilter = StaticFilter::new();
static FOUND_AUTHORS: StaticCell<bool> = StaticCell::new(false);

/// Build a filter capturing reposts and reactions near a target timestamp.
fn init_releated_filter(target_ts: usize, time_window: Option<usize>, limit: Option<usize>) {
    let time_window = time_window
        .and_then(|days| days.checked_mul(24 * 60 * 60))
        .unwrap_or(SEVEN_DAYS);

    RELATED_FILTER.kind(REPOST_KIND);
    RELATED_FILTER.kind(REACTION_KIND);
    RELATED_FILTER.since(target_ts - time_window);
    RELATED_FILTER.until(target_ts + time_window);
    RELATED_FILTER.close_on_eose();

    if let Some(limit) = limit {
        RELATED_FILTER.limit(limit);
    }
}

#[nostr_scrolls::main]
fn run(
    event: Event,
    #[from(Option<PositiveNumber>)] time_window: Option<usize>,
    #[from(Option<PositiveNumber>)] limit: Option<usize>,
) {
    nostr_scrolls::log("resonate: loading target event");

    init_releated_filter(event.created_at(), time_window, limit);

    // Find likes and reposts for this event
    let like_re_sub = Filter::new()
        .kind(REPOST_KIND)
        .kind(REACTION_KIND)
        .tag('e', event.id_hex())
        .tag('p', event.pubkey_hex())
        .close_on_eose()
        .subscribe();

    // Add authors of likes/reposts to filter
    like_re_sub.on_event(cb!(|event| {
        *FOUND_AUTHORS.get_mut() = true;
        RELATED_FILTER.author(&event.pubkey())
    }));

    // Subscribe and display events liked by those authors
    like_re_sub.on_eose(cb!(|| {
        if !*FOUND_AUTHORS.get() {
            panic!("No likes or reposts found for the target event");
        }
        RELATED_FILTER.subscribe().on_event(cb!(|e| display(&e)))
    }));
}
