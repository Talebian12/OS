// src/gdt.rs

/* IMPORTS */

use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;

/// CONSTANTS

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // Interrupt Stack Table Index

/* GDT */

lazy_static! {
    /// CREATE GDT    
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        
        // ADD ENTRIES FOR GDT
        let mut code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let mut tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector
            },
        )
    };

}

pub fn init() {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

// SELECTORS

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector:  SegmentSelector,
}

/* TSS */

lazy_static! {
    /// CREATE TSS
    static ref TSS: TaskStateSegment = {

        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5; // 4M * 5
            
            // STACK
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe{&STACK});
            let stack_end   = stack_start + STACK_SIZE;


            stack_end
        };
        tss
    };
}


