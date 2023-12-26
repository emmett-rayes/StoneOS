pub trait Boot {
    const BOOT_CORE_ID: u64;
}

#[no_mangle]
pub fn start_kernel() -> ! {
    crate::kernel::main();
}
