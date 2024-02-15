use crate::bsp::Bsp;
use core::ops::{Deref, DerefMut};
use lazy_static::lazy_static;
use spin::mutex::SpinMutex;

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

pub type IConsole = impl Read + Write + Send;

pub struct ConsoleWrapper(IConsole);

impl Deref for ConsoleWrapper {
    type Target = IConsole;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConsoleWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

lazy_static! {
    pub static ref CONSOLE: SpinMutex<ConsoleWrapper> =
        SpinMutex::new(ConsoleWrapper(Bsp::console()));
}
