use lazy_static::lazy_static;

use crate::bsp::Bsp;
use crate::sync::mutex::SpinMutex;

pub trait Read {
    fn read(&self) -> char;
}

pub trait Write {
    fn write(&mut self, char: char);
}

pub trait Console {
    fn console() -> impl Read + Write;
}

pub type ConsoleImpl = impl Read + Write + Sync;

pub struct ConsoleWrapper(pub ConsoleImpl);
lazy_static! {
    pub static ref CONSOLE: SpinMutex<ConsoleWrapper> =
        SpinMutex::new(ConsoleWrapper(Bsp::console()));
}
