use crate::boot::BootInfo;
use crate::cpu::park_core;
use crate::println;

pub fn main(_boot_info: BootInfo) -> ! {
    println!("Hello, world!");
    park_core();
}
