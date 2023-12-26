use aarch64_cpu::asm;

use crate::arch::aarch64::Aarch64;
use crate::cpu::Cpu;

impl Cpu for Aarch64 {
    fn park_core() -> ! {
        loop {
            asm::wfe();
        }
    }
}
