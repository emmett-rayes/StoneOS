use crate::console::{Read, Write};

pub struct NullConsole;

impl Read for NullConsole {
    fn read(&mut self) -> char {
        '\0'
    }
}

impl Write for NullConsole {
    fn write(&mut self, _char: char) {}
}
