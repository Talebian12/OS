// src/main.rs

/* FEATURES */

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

/* START */

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    
    // START TEST <- CARGO TEST
    #[cfg(test)]
    test_main();

    loop {}
}

/* PANIC */

// DEFAULT PANIC
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// TEST MODE PANIC
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

