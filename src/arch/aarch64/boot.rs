use core::arch::global_asm;

use crate::boot::Boot;
use crate::bsp::Bsp;

#[no_mangle]
#[link_section = ".rodata.boot_args"]
pub static CORE_ID_MASK: u64 = 0b11;

#[no_mangle]
#[link_section = ".rodata.boot_args"]
pub static BOOT_CORE_ID: u64 = Bsp::BOOT_CORE_ID;

global_asm!(include_str!("boot/boot.asm"));
