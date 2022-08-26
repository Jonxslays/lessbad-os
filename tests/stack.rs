#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

use lessbad::exit_qemu;
use lessbad::serial_print;
use lessbad::serial_println;
use lessbad::QemuExitCode;

pub fn init_test_idt() {
    TEST_IDT.load();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(lessbad::gdt::DOUBLE_FAULT_IST_IDX);
        }

        idt
    };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack::stack_overflow...\t");

    lessbad::gdt::init();

    init_test_idt();
    stack_overflow();

    panic!("Execution was not halted on stack overflow!");
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    // The return address of each recursion is pushed on to the stack
    // causing a stack overflow.
    stack_overflow();

    // Prevent tail recursion optimizations (thanks phil)
    volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lessbad::test_panic_handler(info)
}
