use core::arch::global_asm;

use crate::arch::Arch;
use crate::boot::bios::BiosBoot;
use crate::bsp::Bsp;
use crate::cpu::Cpu;

#[no_mangle]
#[link_section = ".rodata.boot_args"]
pub static CORE_ID_MASK: u64 = Arch::CORE_ID_MASK;

#[no_mangle]
#[link_section = ".rodata.boot_args"]
pub static BOOT_CORE_ID: u64 = Bsp::BOOT_CORE_ID;

global_asm!(include_str!("bios/boot.asm"));
