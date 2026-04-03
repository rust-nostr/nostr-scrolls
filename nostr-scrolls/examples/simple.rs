use nostr_scrolls::Filter;

#[nostr_scrolls::main]
fn run() {
    let mut filter = Filter::new();
    filter.tag('t', "asknostr").unwrap();
    let sub = filter.subscribe();
    sub.on_event(|event, _| {
        if event.kind() == 1 {
            nostr_scrolls::display(&event);
        }
        false // do not close the sub
    });

    sub.on_eose(|| {
        nostr_scrolls::log("Bey").unwrap();
        true // close the sub
    });
}
