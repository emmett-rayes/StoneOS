#[cfg(feature = "boot_bios")]
pub mod bios;

#[cfg(target_os = "uefi")]
#[cfg(feature = "boot_uefi")]
pub mod uefi;

pub struct BootInfo;
