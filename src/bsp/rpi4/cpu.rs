use crate::bsp::rpi4::RPi4;
use crate::cpu::Cpu;

impl Cpu for RPi4 {
    const CLOCK_SPEED: usize = 1_500_000_000;
}
