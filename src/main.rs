#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lessbad::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use lessbad::println;

#[no_mangle]
pub extern "C" fn _start() {
    println!("LessbadOS - Where the bad is less");

    lessbad::init();

    #[cfg(test)]
    test_main();

    println!("Finished successfully.");

    // TODO: Actually do OS stuff
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use lessbad::eprintln;
    eprintln!("Unrecoverable error\n-- {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lessbad::test_panic_handler(info)
}
