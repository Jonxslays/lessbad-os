use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use x86_64::instructions::interrupts;

const SERIAL_PORT_1: u16 = 0x3F8;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut port = unsafe { SerialPort::new(SERIAL_PORT_1) };
        port.init();
        Mutex::new(port)
    };
}

#[doc(hidden)]
pub fn _serial_print(args: ::core::fmt::Arguments) {
    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial device failed");
    });
}
