pub(crate) mod boot;
pub(crate) mod console;
pub(crate) mod cpu;
pub(crate) mod drivers;
pub(crate) mod memory;

pub struct RPi4;

impl RPi4 {
    const UART_BAUD: usize = 115200;
    const UART_CLOCK: usize = 48_000_000;
    const PL011_UART0_START: usize = 0xFE20_1000;
}
