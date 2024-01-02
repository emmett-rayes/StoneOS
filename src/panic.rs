//! Until the bootloader is compiled separately from the kernel,
//! a panic handler is needed only for bios boot as the uefi crate comes with its own panic handler.

#[cfg(not(feature = "boot_uefi"))]
use core::panic::PanicInfo;

#[cfg(not(feature = "boot_uefi"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
