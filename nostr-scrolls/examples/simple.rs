#![no_std]
#![no_main]

use heapless::format;
use nostr_scrolls::Filter;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[nostr_scrolls::main]
fn run() {
    let mut filter = Filter::new();
    filter.tag('t', "asknostr").unwrap();
    filter.send_to("wss://relay.damus.io").unwrap();

    let sub = filter.subscribe();
    sub.on_event(|event, _| {
        if event.kind() == 1 {
            nostr_scrolls::log(&format!(100; "Found {}", event.id_hex()).unwrap()).unwrap();
            nostr_scrolls::display(&event);
        }
        true // close on first event
    });
}
