// src/interrupts.rs

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use crate::println;
use crate::gdt;


/* INIT INTERRUPT DESCRIPTOR TABLE */

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new(); // IDT new instance of InterruptDescriptorTable
        idt.breakpoint.set_handler_fn(breakpoint_handler); // set breakpoint_handler as handler
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load(); // load IDT
}

/// PRINT EXCEPTION - X86-INTERRUPT

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// PRINT EXCEPTION DOUBLE FAULT - X86-INTERRUPT

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

/* TESTS */

// breakpoint exception
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3(); // int3
}


