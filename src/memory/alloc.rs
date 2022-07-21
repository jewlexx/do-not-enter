use core::cell::UnsafeCell;

use linked_list_allocator::LockedHeap;

// Symbols from the linker script.
extern "Rust" {
    static __heap_start: UnsafeCell<u8>;
    static __heap_end_exclusive: UnsafeCell<u8>;
}

#[global_allocator]
pub(super) static KERNEL_HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn init_heap() {
    let heap_start = __heap_start.get();
    let heap_end = __heap_end_exclusive.get() as usize;
    let heap_size = heap_end - heap_start as usize;

    KERNEL_HEAP_ALLOCATOR.lock().init(heap_start, heap_size);
}
