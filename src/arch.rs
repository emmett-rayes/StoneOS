use crate::cpu::Cpu;

#[cfg(target_arch = "aarch64")]
use crate::arch::aarch64::Aarch64 as ArchImpl;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

trait IArch: Cpu {}

impl IArch for ArchImpl {}

pub type Arch = ArchImpl;
