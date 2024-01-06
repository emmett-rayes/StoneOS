use crate::bsp::rpi4::drivers::pl011::Pl011;
use crate::console::{Read, Write};

impl Read for Pl011 {
    fn read(&mut self) -> char {
        self.read_byte() as char
    }
}

impl Write for Pl011 {
    fn write(&mut self, char: char) {
        self.write_byte(char as u8)
    }
}
