use crate::bsp::rpi4::drivers::pl011::Pl011;
use crate::bsp::rpi4::RPi4;
use crate::console::{Console, Read, Write};
use crate::memory::address::{Address, PhysicalAddress};

pub(super) mod p0l11;

impl Console for RPi4 {
    fn console() -> impl Read + Write {
        // Safety: this is the only place where the raw address is used.
        let mut console = unsafe { Pl011::new(PhysicalAddress::new(Self::PL011_UART0_START)) };
        console.init(Self::UART_CLOCK, Self::UART_BAUD);
        console
    }
}
