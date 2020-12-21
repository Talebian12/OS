// src/main.rs

/* FEATURES */

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use x86_64::structures::paging::PageTable;
use bootloader::{ BootInfo, entry_point};
use core::panic::PanicInfo;
use alloc::{
    boxed::Box,
    vec,
    vec::Vec,
    rc::Rc
};
use os::println;

/// SET ENTRYPOINT

entry_point!(kernel_main);

/* START */

fn kernel_main(boot_info: &'static BootInfo) -> ! 
{
    use os::memory;
    use os::allocator;
    use os::memory::BootInfoFrameAllocator;
    use x86_64::{
        structures::paging::Page,
        VirtAddr
    };

    println!("Hello World!");
    
    os::init(); // INIT GDT AND IDT;
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { 
        page_ptr.offset(400)
                .write_volatile(0x_f021_f077_f065_f04e)
    };
   

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    /// TEST HEAP 
    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    // START TEST <- CARGO TEST
    #[cfg(test)]
    test_main();
    

    // ALL HAS BEEN EXECUTED, KERNEL DIDN'T CRASH
    println!("It didn't crash!");
    os::hlt_loop();
}

/* PANIC */

// DEFAULT PANIC
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

// TEST MODE PANIC
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
