use crate::boot::bios::BiosBoot;
use crate::bsp::rpi4::RPi4;

impl BiosBoot for RPi4 {
    const BOOT_CORE_ID: u64 = 0;
}
