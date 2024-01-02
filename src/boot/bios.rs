use crate::boot::BootInfo;

pub trait BiosBoot {
    const BOOT_CORE_ID: u64;
}

#[no_mangle]
pub fn start() -> ! {
    crate::kernel::main(BootInfo);
}
