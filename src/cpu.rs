use crate::arch::Arch;
use crate::bsp::Bsp;

pub trait Cpu {
    const CLOCK_SPEED: usize = Bsp::CLOCK_SPEED;
    const CORE_ID_MASK: u64 = Arch::CORE_ID_MASK;
}

pub fn park_core() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
