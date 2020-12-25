// src/allocator/bump.rs



pub struct BumpAllocator {
    heap_start:  usize,
    heap_end:    usize,
    allocations: usize,
    next:        usize,
}

impl BumpAllocator {
    /// NEW BUMP ALLOCATOR
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start:  0,
            heap_end:    0,
            allocations: 0,
            next:        0,
        }
    }
}
