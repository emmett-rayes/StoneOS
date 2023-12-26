#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::Aarch64 as Arch;
