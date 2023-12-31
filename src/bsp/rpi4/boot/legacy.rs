use crate::boot::legacy::LegacyBoot;
use crate::bsp::rpi4::RPi4;

impl LegacyBoot for RPi4 {
    const BOOT_CORE_ID: u64 = 0;
}
