# Nostr Scrolls

Self-contained WebAssembly programs packaged as Nostr events ("scrolls"). The
binaries execute in a sandboxed environment inside a "host" (i.e., a proper
Nostr client). Scrolls interact with Nostr exclusively through a minimal set of
APIs provided by the host.

This SDK provides a safe, ergonomic wrapper around the host API, making scroll
development straightforward. It includes the `nostr_scrolls::main` macro, which
handles parameter deserialization from the host-supplied pointer and registers a
panic handler that logs errors before termination.

## Getting started

```rust,no_run
#![no_std]

extern crate alloc;

use nostr_scrolls::{PublicKey, Filter, cb};

#[allow(unused_must_use)]
#[nostr_scrolls::main]
fn run(me: PublicKey) {
  let filter = Filter::new();
  filter.author(&me);
  filter.kind(1);
  filter.close_on_eose();
  filter.tag('t', "asknostr");

  let sub = filter.subscribe();
  sub.on_event(cb!(|event| nostr_scrolls::display(&event)));
}
```

More examples can be found in the [examples directory](./examples).

## Features

### `debug-strings`

This crate does not use `.unwrap()` or `.expect()` and by default the structs
don't implement `Debug` nor `Display` traits. These functions and traits depend
on `core::fmt`, and its instruction set is huge, it easily adds up `5~8KiB` to
your wasm program. Anyway, the runtime will not see those debug information, so
they are disabled by default, if you need them just enable the feature

Relays check the nostr event size, they usually limit it to 16KiB, just make
sure your program doesn't exceed this limit, `core::fmt` is useless in our case,
make sure your program doesn't load it. You can check the program `wat` in debug
and search for `4core3fmt`. If it's there fix your program.

## Global allocator

This crate creates a global bump allocator, so your std code or `alloc`
will use it. The bump allocator is simple, you can find it here:
[allocator.rs](./nostr-scrolls/src/allocator.rs). This allocator just pushes the
WASM linear memory; it doesn't free memory, so don't overuse it.

## Changelog

All notable changes to this library are documented in the [CHANGELOG.md](CHANGELOG.md).

## State

**This library is in an ALPHA state**, things that are implemented generally
work but the API will change in breaking ways.

## Donations

`rust-nostr` is free and open-source. This means we do not earn any
revenue by selling it. Instead, we rely on your financial support. If you
actively use any of the `rust-nostr` libs/software/services, then please
[donate](https://rust-nostr.org/donate).

## License

This project is distributed under the MIT software license - see the
[LICENSE](LICENSE) file for details

