#![no_std]
#![no_main]

use core::panic::PanicInfo;

use lessbad::exit_qemu;
use lessbad::serial_print;
use lessbad::serial_println;
use lessbad::QemuExitCode;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_panic();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_panic() {
    serial_print!("panic::should_panic...\t\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
