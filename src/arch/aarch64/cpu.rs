use crate::arch::aarch64::Aarch64;
use crate::cpu::Cpu;

impl Cpu for Aarch64 {
    const CORE_ID_MASK: u64 = 0b11;
}
