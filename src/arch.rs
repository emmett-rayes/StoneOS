#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::Aarch64 as Arch;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
