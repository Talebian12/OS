// src/main.rs

/* FEATURES */

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

/* MODULES */

mod vga_buffer; // VGA MODULE
mod serial;     // SERIAL MODULE

/* START */

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World!");
    
    // START TEST <- CARGO TEST
    #[cfg(test)]
    test_main();

    // GENERATE PANIC
    panic!("VAFFANMOCC");

    loop {}
}

/* PANIC */

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

/* I/O */

// QEMU EXIT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    SUCCESS  =  0x10,   // SUCCESS EXIT CODE
    FAILED   =  0x11,   // FAILED  EXIT CODE
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4); // set port 0xf4
        port.write(exit_code as u32); // EXIT WITH exit_code
    }
}

/* TESTS */

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("======== TEST SERIAL CONSOLE ========");
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    // EXIT SUCCESS
    exit_qemu(QemuExitCode::SUCCESS);
}

// TEST
#[test_case]
fn trivial_assertion() {
    serial_print!("> trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}
