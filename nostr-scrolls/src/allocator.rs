// Copyright (c) 2026 Rust Nostr Developers
// Distributed under the MIT software license

use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
};

/// Size of a WASM memory page in bytes (64 KiB).
const PAGE_SIZE: usize = 65536;
/// Default align
const ALIGN: usize = 8;

/// A bump allocator optimized for short-lived WASM programs.
///
/// Memory grows monotonically and is never freed. The allocator starts at
/// `__heap_base` and expands WASM linear memory on demand.
struct BumpAllocator;

/// A wrapper to allow interior mutability in a static context.
struct SingleThreadedState(UnsafeCell<usize>);
unsafe impl Sync for SingleThreadedState {}

/// Next available address for allocation. Zero indicates uninitialized state.
static BUMP_ADDR: SingleThreadedState = SingleThreadedState(UnsafeCell::new(0));

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let bump_ptr = BUMP_ADDR.0.get();
        let mut current_addr = *bump_ptr;

        // Initialize lazily on the first allocation
        if current_addr == 0 {
            current_addr = core::ptr::addr_of!(__heap_base) as usize;
        }

        // Align the current address
        let aligned_addr = current_addr.next_multiple_of(align);
        let Some(next_addr) = aligned_addr.checked_add(size) else {
            return core::ptr::null_mut();
        };

        let current_mem_bytes = core::arch::wasm32::memory_size::<0>() * PAGE_SIZE;

        // Grow memory if needed
        if next_addr > current_mem_bytes {
            let bytes_needed = next_addr - current_mem_bytes;
            let pages_to_grow = bytes_needed.div_ceil(PAGE_SIZE);

            if core::arch::wasm32::memory_grow::<0>(pages_to_grow) == usize::MAX {
                return core::ptr::null_mut();
            }
        }

        // Store the next available address
        *bump_ptr = next_addr;

        aligned_addr as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op for bump allocators
    }
}

#[global_allocator]
static BUMP_ALLOC: BumpAllocator = BumpAllocator;

#[unsafe(no_mangle)]
#[doc(hidden)]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return core::ptr::null_mut();
    }

    let layout = match Layout::from_size_align(size, ALIGN) {
        Ok(l) => l,
        Err(_) => return core::ptr::null_mut(),
    };

    BUMP_ALLOC.alloc(layout)
}

unsafe extern "C" {
    unsafe static __heap_base: u8;
}
