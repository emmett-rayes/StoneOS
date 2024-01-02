use crate::arch::Arch;
use crate::boot::BootInfo;
use crate::cpu::Cpu;

pub fn main(_boot_info: BootInfo) -> ! {
    Arch::park_core();
}
