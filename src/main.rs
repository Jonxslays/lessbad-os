#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lessbad::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use x86_64::VirtAddr;

use lessbad::allocator;
use lessbad::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("LessbadOS - Where the bad is less");
    println!("Initializing...");
    lessbad::init();

    // Initialize the heap
    let phys_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { allocator::init(phys_offset) };
    let mut frame_allocator = unsafe { allocator::BootFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed.");

    #[cfg(test)]
    test_main();

    println!("Initialized  successfully.");
    // TODO: Actually do OS stuff
    lessbad::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use lessbad::eprintln;
    eprintln!("Unrecoverable error\n-- {}", info);
    lessbad::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lessbad::test_panic_handler(info)
}
