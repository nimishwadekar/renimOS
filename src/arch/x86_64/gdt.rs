use x86_64::{
    structures::{
        tss::TaskStateSegment,
        gdt::{
            GlobalDescriptorTable,
            Descriptor,
            SegmentSelector
        }
    },
    VirtAddr,
    registers::segmentation::{
        CS,
        Segment, SS, DS, ES
    },
    instructions::tables::load_tss
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub fn init() {
    static mut TSS: TaskStateSegment = TaskStateSegment::new();
    unsafe {
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE]; // Replace with dynamic memory when possible.
            let stack_start = VirtAddr::from_ptr(&STACK);
            stack_start + STACK_SIZE
        };
    }

    static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

    let selectors = unsafe {
        let kernel_code_selector = GDT.add_entry(Descriptor::kernel_code_segment());
        let kernel_data_selector = GDT.add_entry(Descriptor::kernel_data_segment());
        let tss_selector = GDT.add_entry(Descriptor::tss_segment(&TSS));

        GDT.load();

        Selectors {
            kernel_code_selector,
            kernel_data_selector,
            tss_selector
        }
    };

    unsafe {
        CS::set_reg(selectors.kernel_code_selector);
        SS::set_reg(selectors.kernel_data_selector);
        DS::set_reg(selectors.kernel_data_selector);
        ES::set_reg(selectors.kernel_data_selector);
        load_tss(selectors.tss_selector);
    }
}

struct Selectors {
    kernel_code_selector: SegmentSelector,
    kernel_data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}