#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lessbad::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use x86_64::VirtAddr;

use lessbad::allocator;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    lessbad::init();

    let phys_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { allocator::init(phys_offset) };
    let mut frame_allocator = unsafe {
        allocator::BootFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    test_main();
    lessbad::hlt_loop();
}

#[test_case]
fn simple_allocation() {
    let left = Box::new(69);
    let right = Box::new(420);

    assert_eq!(*left, 69);
    assert_eq!(*right, 420);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut container = Vec::new();

    for i in 0..n {
        container.push(i);
    }

    assert_eq!(container.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..(100 * 1024) {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lessbad::test_panic_handler(info)
}
