use lazy_static::lazy_static;

use crate::bsp::Bsp;
use spin::mutex::SpinMutex;

mod null_console;
mod print;

pub trait Read {
    fn read(&self) -> char;
}

pub trait Write {
    fn write(&mut self, char: char);
}

pub trait Console {
    fn console() -> impl Read + Write;
}

type ConsoleImpl = impl Read + Write + Send + Sync;

pub struct ConsoleWrapper(ConsoleImpl);
lazy_static! {
    pub static ref CONSOLE: SpinMutex<ConsoleWrapper> =
        SpinMutex::new(ConsoleWrapper(Bsp::console()));
}
