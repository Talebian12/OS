// src/interrupts.rs

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::println;


/* INIT INTERRUPT DESCRIPTOR TABLE */

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); // IDT new instance of InterruptDescriptorTable
        idt.breakpoint.set_handler_fn(breakpoint_handler); // set breakpoint_handler as handler
        idt
    };
}

pub fn init_idt() {
    IDT.load(); // load IDT
}

/// PRINT EXCEPTION - X86-INTERRUPT

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}


/* TESTS */

// breakpoint exception
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3(); // int3
}


