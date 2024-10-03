use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;
use crate::println;

pub struct BumpAllocator {
    heap_end: usize,
    next: UnsafeCell<usize>
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        BumpAllocator {
            heap_end: heap_size + heap_start,
            next: UnsafeCell::new(heap_start)
        }
    }
}

unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_start = align_up(*self.next.get(), layout.align());
        let alloc_end = alloc_start.checked_add(layout.size()).expect("Error allocating heap size. (Overflow)");

        println!("Heap start: {:x}, Heap end: {:x}", *self.next.get(), self.heap_end);
        println!("Trying to allocate {:x}", layout.size());
        println!("Align: {:x}", layout.align());
        println!("New alloc pointer: {:x}", alloc_end);

        if alloc_end > self.heap_end {
            ptr::null_mut()
        } else {
            *self.next.get() = alloc_end;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("Dealloc not implemented");
        // No deallocation!
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new(0x100_0000, 1024 * 1024);