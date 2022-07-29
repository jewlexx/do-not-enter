use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
};

use linked_list_allocator::Heap as LinkedListHeap;
use spin::Mutex;

// Symbols from the linker script.
extern "Rust" {
    static __heap_start: UnsafeCell<u8>;
    static __heap_end_exclusive: UnsafeCell<u8>;
}

/// A heap allocator that can be lazyily initialized.
pub struct HeapAllocator {
    inner: Mutex<LinkedListHeap>,
}

#[global_allocator]
pub(super) static KERNEL_HEAP_ALLOCATOR: HeapAllocator = HeapAllocator::new();

impl HeapAllocator {
    /// Create an instance.
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(LinkedListHeap::empty()),
        }
    }
}

unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let result = KERNEL_HEAP_ALLOCATOR
            .inner
            .lock()
            .allocate_first_fit(layout)
            .ok();

        match result {
            None => core::ptr::null_mut(),
            Some(allocation) => allocation.as_ptr(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        KERNEL_HEAP_ALLOCATOR
            .inner
            .lock()
            .deallocate(core::ptr::NonNull::new_unchecked(ptr), layout);
    }
}

/// Query the BSP for the heap region and initialize the kernel's heap allocator with it.
///
/// # Safety
///
/// - This function must be called exactly once.
/// - This function resolves the heap region from the BSP.
pub unsafe fn kernel_init_heap_allocator() {
    let heap_start = __heap_start.get();
    let heap_end = __heap_end_exclusive.get() as usize;
    let heap_size = heap_end - heap_start as usize;

    KERNEL_HEAP_ALLOCATOR
        .inner
        .lock()
        .init(heap_start, heap_size);
}
