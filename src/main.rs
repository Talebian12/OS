// src/main.rs

/* FEATURES */

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{ bootloader., entry_point};
use core::panic::PanicInfo;
use os::println;

/// SET ENTRYPOINT

entry_point!(kernel_main);

/* START */

fn kernel_main(
    boot_info: &'static BootInfo) -> ! 
{
    println!("Hello World!");
    
    os::init(); // INIT GDT AND IDT;
    
    use x86_64::registers::control::Cr3; 

    // START TEST <- CARGO TEST
    #[cfg(test)]
    test_main();
    
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
