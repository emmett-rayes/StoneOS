use crate::bsp::rpi4::RPi4;
use crate::console::{Console, Read, Write};

struct NullConsole;

impl Read for NullConsole {
    fn read(&self) -> char {
        ' '
    }
}

impl Write for NullConsole {
    fn write(&mut self, _char: char) {}
}

impl Console for RPi4 {
    fn console() -> impl Read + Write {
        NullConsole {}
    }
}
