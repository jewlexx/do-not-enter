pub extern crate alloc as core_alloc;

use core::mem::MaybeUninit;

use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

// TODO: Ensure that this is correct
const HEAP_SIZE: usize = 1024;

static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

pub unsafe fn init_alloc() {
    ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE);
}
