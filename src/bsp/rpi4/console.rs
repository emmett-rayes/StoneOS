use crate::bsp::rpi4::drivers::pl011::Pl011;
use crate::bsp::rpi4::memory::memory_map::PL011_UART0_START;
use crate::bsp::rpi4::RPi4;
use crate::console::{Console, Read, Write};

pub mod p0l11;

impl Console for RPi4 {
    fn console() -> impl Read + Write {
        let mut console = unsafe { Pl011::new(PL011_UART0_START, 921_600) };
        console.init();
        console
    }
}
