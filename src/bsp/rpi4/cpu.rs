use crate::arch::Arch;
use crate::bsp::rpi4::RPi4;
use crate::cpu::Cpu;

impl Cpu for RPi4 {
    const CLOCK_SPEED: usize = 1_500_000_000;
    const CORE_ID_MASK: u64 = Arch::CORE_ID_MASK;
}
