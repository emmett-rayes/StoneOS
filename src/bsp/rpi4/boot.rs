use crate::boot::Boot;
use crate::bsp::rpi4::RPi4;

impl Boot for RPi4 {
    const BOOT_CORE_ID: u64 = 0;
}
