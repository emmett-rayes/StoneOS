use crate::bsp::rpi4::drivers::pl011::Pl011;
use crate::bsp::rpi4::memory::memory_map::PL011_UART0_START;
use crate::bsp::rpi4::RPi4;
use crate::bsp::Bsp;
use crate::console::{Console, Read, Write};
use crate::memory::address::{Address, PhysicalAddress};

pub(super) mod p0l11;

impl Console for RPi4 {
    fn console() -> impl Read + Write {
        // Safety: this is the only place where the raw address is used.
        let mut console = unsafe { Pl011::new(PhysicalAddress::new(PL011_UART0_START)) };
        console.init(Bsp::UART_CLOCK, Bsp::UART_BAUD);
        console
    }
}
