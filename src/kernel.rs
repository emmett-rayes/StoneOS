use crate::arch::Arch;
use crate::cpu::Cpu;

pub fn main() -> ! {
    Arch::park_core();
}
