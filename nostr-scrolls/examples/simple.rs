//! Display all Kind 1 (text note) posts tagged with "#asknostr" authored by the
//! current user (the one running this program).
//!
//! Program parameters:
//! ```
//! [
//!   ["param", "me", "", "public_key", "required"],
//!   ["param", "limit", "limit the notes number", "number", ""]
//!   ["param", "relay", "The relay to get the notes from", "relay", ""]
//! ]
//! ```

#![no_std]
#![no_main]

use heapless::format;
use nostr_scrolls::{Filter, PublicKey};

#[allow(unused_must_use)]
#[nostr_scrolls::main]
fn run(me: PublicKey, mut limit: Option<i32>, relay: Option<&str>) {
    nostr_scrolls::log("Running simple example: Fetching your notes tagged with #asknostr");

    let mut filter = Filter::new();
    filter.author(&me);
    filter.kind(1);
    filter.close_on_eose();
    filter.tag('t', "asknostr");

    limit = limit.map(|l| l + 1);
    if let Some(limit) = limit {
        filter.limit(limit as usize);
    }

    if let Some(relay) = relay {
        filter.send_to(relay);
    }

    let sub = filter.subscribe();
    sub.on_event(|event, _| {
        nostr_scrolls::log(&format!(100; "Found {}", event.id_hex()).unwrap()).unwrap();
        nostr_scrolls::display(&event);
        false
    });
}
