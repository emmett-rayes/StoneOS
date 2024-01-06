use lazy_static::lazy_static;
use spin::mutex::SpinMutex;

use crate::bsp::Bsp;

mod null_console;
mod print;

pub trait Read {
    fn read(&mut self) -> char;
}

pub trait Write {
    fn write(&mut self, char: char);
}

pub trait Console {
    fn console() -> impl Read + Write;
}

type ConsoleImpl = impl Read + Write + Send;

pub struct ConsoleWrapper(ConsoleImpl);
lazy_static! {
    pub static ref CONSOLE: SpinMutex<ConsoleWrapper> =
        SpinMutex::new(ConsoleWrapper(Bsp::console()));
}
