// src/allocator.rs

use x86_64::{
    structures::paging::{
        mapper::MapToError,
        FrameAllocator,
        Mapper,
        Page,
        PageTableFlags, 
        Size4KiB,
    },
    VirtAddr,
};
use alloc::alloc::{
    GlobalAlloc,
    Layout,
};
use linked_list_allocator::LockedHeap;
use core::ptr::null_mut;
use bump::BumpAllocator;

pub mod bump;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());


/// DUMMY ALLOCATOR

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    /// ALLOC
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    /// DEALLOC
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}

/// WRAPPER AROUND MUTEX TO PERMIT TRAIT IMPL

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

/// ALIGN
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align -1) & !(align - 1)
}

/// INIT

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end   = heap_start + HEAP_SIZE - 1u64;
        
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page   = Page::containing_address(heap_end);

        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
                        .allocate_frame()
                        .ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::PRESENT |
                    PageTableFlags::WRITABLE;

        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
            ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
        };
    }

    Ok(())

}
