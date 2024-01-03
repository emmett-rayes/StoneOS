use crate::boot::BootInfo;
use crate::cpu::park_core;

pub fn main(_boot_info: BootInfo) -> ! {
    park_core();
}
