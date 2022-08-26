/// Print with no newline at the end.
///
/// Ex:
///
/// ```
/// let hello = "hello";
/// print!("{}, world!", hello);
/// print!("{}", hello);
/// print!(hello);
/// ```
#[macro_export]
macro_rules! print {
    // This is inefficient but prevents compiler errors
    // when someone calls `print!()` with no args
    () => {{
        let (l, c) = (line!(), column!());
        $crate::warn!(
            "Warning: invalid print! call on line: {}, column: {}", l, c
        );
    }};
    ($($arg:ident)*) => { $crate::vga::_print(format_args!("{}",$($arg)*)) };
    ($($arg:tt)*) => { $crate::vga::_print(format_args!($($arg)*)) };
}

/// Print with a newline at the end.
///
/// Ex:
///
/// ```
/// let hello = "hello";
/// println!("{}, world!", hello);
/// println!("{}", hello);
/// println!(hello);
/// ```
#[macro_export]
macro_rules! println {
    () => { print!("\n") };
    ($($arg:tt)*) => { $crate::print!("{}\n", format_args!($($arg)*)) };
}

/// Print in yellow with a newline at the end
///
/// Ex:
///
/// ```
/// let hello = "hello";
/// wprintln!("{}, world!", hello);
/// wprintln!("{}", hello);
/// wprintln!(hello);
/// ```
#[macro_export]
macro_rules! wprintln {
    () => { print!("\n") };
    ($($arg:tt)*) => {

        $crate::vga::WRITER.lock().set_fg(
            $crate::vga::Color::Yellow
        );

        print!("{}\n", format_args!($($arg)*));

        $crate::vga::WRITER.lock().set_fg(
            $crate::vga::Color::White
        );
    };
}

/// Print in red with a newline at the end
///
/// Ex:
///
/// ```
/// let hello = "hello";
/// eprintln!("{}, world!", hello);
/// eprintln!("{}", hello);
/// eprintln!(hello);
/// ```
#[macro_export]
macro_rules! eprintln {
    () => { print!("\n") };
    ($($arg:tt)*) => {
        $crate::vga::WRITER.lock().set_fg(
            $crate::vga::Color::Red
        );

        $crate::print!("{}\n", format_args!($($arg)*));

        $crate::vga::WRITER.lock().set_fg(
            $crate::vga::Color::White
        );
    };
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => { $crate::serial_print!("\n") };
    ($fmt:expr) => { $crate::serial_print!(concat!($fmt, "\n")) };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::serial_print!(concat!($fmt, "\n"), $($arg)*)
    };
}
