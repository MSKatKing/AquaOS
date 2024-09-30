use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: UnsafeCell<usize>
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end: heap_size + heap_start,
            next: UnsafeCell::new(heap_start)
        }
    }
}

unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_start = self.next.get();

        let align = layout.align();
        let aligned_start = (*alloc_start + align - 1) & !(align - 1);

        if aligned_start + layout.size() > self.heap_end {
            ptr::null_mut()
        } else {
            *alloc_start = aligned_start + layout.size();
            aligned_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No deallocation!
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new(0x4444_0000, 100 * 1024);