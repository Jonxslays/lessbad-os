use lazy_static::lazy_static;
use x86_64::instructions::segmentation::Segment;
use x86_64::instructions::segmentation::CS;
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::Descriptor;
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_IDX: u16 = 0;
pub const IST_STACK_SIZE: usize = 4096 * 5;

pub fn init() {
    GDT_WRAPPER.gdt.load();

    unsafe {
        CS::set_reg(GDT_WRAPPER.selectors.code_selector);
        load_tss(GDT_WRAPPER.selectors.tss_selector);
    }
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DOUBLE_FAULT_IST_IDX as usize] = {
            static mut STACK: [u8; IST_STACK_SIZE] = [0; IST_STACK_SIZE];

            let stack_begin = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_begin + IST_STACK_SIZE;

            stack_end
        };

        tss
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

struct GdtWrapper {
    gdt: GlobalDescriptorTable,
    selectors: Selectors,
}

lazy_static! {
    static ref GDT_WRAPPER: GdtWrapper = {
        let mut gdt = GlobalDescriptorTable::new();

        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        let selectors = Selectors { code_selector, tss_selector };

        GdtWrapper { gdt, selectors }
    };
}
