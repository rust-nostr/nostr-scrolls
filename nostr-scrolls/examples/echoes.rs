//! Recommends events based on shared likes with similar users
//!
//! Program parameters:
//! ```
//! [
//!   ["param", "me", "Your public key", "public_key", "required"],
//!   ["param", "Likes Limit", "Maximum number of your likes to fetch. Default: 10", "number", ""],
//!   ["param", "Shared Likes Threshold", "Minimum number of shared likes required to recommend an event. Default: 2", "number", ""],
//!   ["param", "Events To Check", "Number of potential events to evaluate. Default: 300", "number", ""],
//!   ["param", "Events To Display", "Max events to display. Default: 100", "number", ""],
//! ]
//! ```

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use nostr_scrolls::{
    Event, EventId, Filter, PositiveNumber, PublicKey, ShortEventId, StaticCell, StaticFilter, cb,
    cb_ret, display, utils,
};

const RELAYS_LIST_KIND: u16 = 10002;
const REPOST_KIND: u16 = 6;
const REACTION_KIND: u16 = 7;
const DEFAULT_LIKES_LIMIT: usize = 10;
const DEFAULT_THRESHOLD: usize = 2;
const DEFAULT_POTENTIAL_LIMIT: usize = 300;
const DEFAULT_TARGETS_LIMIT: usize = 100;

/// Tracks whether the current user has liked any event.
static FOUND_LIKES: StaticCell<bool> = StaticCell::new(false);

/// Locates READ relays of authors whose events "me" has liked, enabling
/// discovery of additional likes on those events.
static INBOXES_REQ: StaticFilter = StaticFilter::init(|f| f.kind(RELAYS_LIST_KIND).close_on_eose());

/// Gathers events liked by "me" to find other users who liked the same events,
/// fetching their likes from authors' READ relays.
static SEED_REQ: StaticFilter =
    StaticFilter::init(|f| f.kind(REACTION_KIND).kind(REPOST_KIND).close_on_eose());

static FOUND_AUTHORS: StaticCell<bool> = StaticCell::new(false);

static POTENTIAL_TARGETS_LIMIT: StaticCell<usize> = StaticCell::new(DEFAULT_POTENTIAL_LIMIT);
/// Tracks authors who reacted to or reposted the same events as "me", we want
/// to see what else they likes
static AUTHORS_REQ: StaticFilter = StaticFilter::init(|f| {
    f.kind(REACTION_KIND)
        .kind(REPOST_KIND)
        .limit(*POTENTIAL_TARGETS_LIMIT.get())
        .close_on_eose()
});

/// Events potentially relevant to the user; promoted to `TARGETS_REQ` if
/// significance exceeds threshold.
static POTENTIAL_TARGETS: StaticCell<Vec<(ShortEventId, usize)>> = StaticCell::new(Vec::new());

static TARGETS_LIMIT: StaticCell<usize> = StaticCell::new(DEFAULT_TARGETS_LIMIT);
static TARGETS_COUNT: StaticCell<usize> = StaticCell::new(0);
/// Events exceeding significance threshold, available for user display.
static TARGETS_REQ: StaticFilter =
    StaticFilter::init(|f| f.limit(*TARGETS_LIMIT.get()).close_on_eose());

/// Processes initialization subscription events
fn init_sub_event(event: Event) {
    let Some(liked_event_id) = event.tag_item_by_name_bytes("e", 1) else {
        return;
    };
    let Some(liked_event_author) = event.tag_item_by_name_bytes("p", 1).map(PublicKey::from) else {
        return;
    };

    *FOUND_LIKES.get_mut() = true;
    // Where we can find the likes
    INBOXES_REQ.author(&liked_event_author);
    // To get this event likes
    SEED_REQ.tag_bytes('e', &liked_event_id);
}

/// Subscribe to `INBOXES_REQ` to collect the READ relays
fn init_sub_eose(me: PublicKey, threshold: usize) {
    if !*FOUND_LIKES.get() {
        panic!("Can't find any event you liked");
    }

    let sub = INBOXES_REQ.subscribe();
    // On each relays list, add the READ relays to the `SEED_REQ`
    sub.on_event(cb!(|event| {
        for relay in utils::read_relays(&event, 8) {
            if relay.is_read() {
                SEED_REQ.send_to(relay.url());
            }
        }
    }));
    sub.on_eose(cb!(|| inboxes_eose(me, threshold)));
}

/// On `INBOXES_REQ` EOSE start the `SEED_REQ` subscription
fn inboxes_eose(me: PublicKey, threshold: usize) {
    let sub = SEED_REQ.subscribe();
    sub.on_event(cb!(|event| {
        if event.pubkey() == me {
            return;
        }
        *FOUND_AUTHORS.get_mut() = true;
        // To see what they also likes
        AUTHORS_REQ.author(&event.pubkey());
    }));

    sub.on_eose(cb!(|| {
        if !FOUND_AUTHORS.get() {
            panic!("No one likes what you likes");
        }
        // TODO: Sub to authors
        let sub = AUTHORS_REQ.subscribe();
        sub.on_event(cb_ret!(|e| authors_event(e, threshold)));
        sub.on_eose(cb!(|| authors_eose()));
    }));
}

/// Scores events by how many authors who share "me"s likes also liked them.
///
/// Events cross `threshold` are promoted to `TARGET_REQ`.
fn authors_event(event: Event, threshold: usize) -> bool {
    // TODO: The author can repost and reaction to the event more than once,
    //       make sure you only count one from each author to the same event
    // TODO: Make sure that "me" did not react to this event by indexing what it likes

    // Close the subscription when the targets limit is reached
    if TARGETS_COUNT.get() >= TARGETS_LIMIT.get() {
        nostr_scrolls::log("Reached the targets limit");
        authors_eose();
        return true;
    }

    // Extract the event being reacted to
    let Some(event_id) = event.tag_item_by_name_bytes("e", 1).map(EventId::from) else {
        return false;
    };

    let mut found = false;
    // Search existing potential targets for a matching short ID
    for (target, score) in POTENTIAL_TARGETS.get_mut() {
        if event_id.matches_short(target) {
            found = true;
            *score += 1;

            // Promote to target request once enough distinct authors have liked it
            if *score >= threshold {
                *TARGETS_COUNT.get_mut() += 1;
                TARGETS_REQ.id(&event_id);
            }
        }
    }

    // First time seeing this event - initialize with score of 1
    if !found {
        POTENTIAL_TARGETS.get_mut().push((event_id.short(), 1));
    }

    false
}

/// Promotes accumulated targets to a request once all author events are received.
fn authors_eose() {
    // TODO: Send the `TARGETS_REQ` to its authors relays

    if *TARGETS_COUNT.get() == 0 {
        panic!("failed to find any event meeting the threshold");
    }

    // Subscribe to all promoted targets and display matching events
    TARGETS_REQ.subscribe().on_event(cb!(|e| display(&e)));
}

#[nostr_scrolls::main]
fn run(
    me: PublicKey,
    #[from(Option<PositiveNumber>)] likes_limit: Option<usize>,
    #[from(Option<PositiveNumber>)] threshold: Option<usize>,
    #[from(Option<PositiveNumber>)] potential_limit: Option<usize>,
    #[from(Option<PositiveNumber>)] targets_limit: Option<usize>,
) {
    // Subscribes to the user's reactions and reposts
    let init_sub = Filter::new()
        .author(&me)
        .kind(REACTION_KIND)
        .kind(REPOST_KIND)
        .limit(likes_limit.unwrap_or(DEFAULT_LIKES_LIMIT))
        .close_on_eose()
        .subscribe();

    let threshold = threshold.unwrap_or(DEFAULT_THRESHOLD);

    *POTENTIAL_TARGETS_LIMIT.get_mut() = potential_limit.unwrap_or(DEFAULT_POTENTIAL_LIMIT);
    *TARGETS_LIMIT.get_mut() = targets_limit.unwrap_or(DEFAULT_TARGETS_LIMIT);
    init_sub.on_event(cb!(|event| init_sub_event(event)));
    init_sub.on_eose(cb!(|| init_sub_eose(me, threshold)));
}
