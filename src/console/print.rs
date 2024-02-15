use core::fmt;

use crate::console::ConsoleWrapper;
use crate::console::Write;

impl fmt::Write for ConsoleWrapper {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for char in s.chars() {
            self.write(char);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (core::fmt::Write::write_fmt(&mut *$crate::console::CONSOLE.lock(), format_args!($($arg)*)).unwrap());
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
