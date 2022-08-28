#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

// Include the allocator from stdlib
extern crate alloc;

#[cfg(test)]
use bootloader::entry_point;
#[cfg(test)]
use bootloader::BootInfo;
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

pub mod descriptors;
pub mod io;
pub mod macros;
pub mod memory;

pub use descriptors::*;
pub use io::*;
pub use memory::*;

pub fn init() {
    gdt::init();
    idt::init();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x22,
    Failed = 0x33472,
}

pub fn exit_qemu(ex_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xF4);
        port.write(ex_code as u32);
    }
}

pub trait TestCase {
    fn run(&self) -> ();
}

impl<T> TestCase for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn TestCase]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}
