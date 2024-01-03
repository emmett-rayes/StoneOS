pub fn park_core() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
