pub trait Cpu {
    const CORE_ID_MASK: u64;
}

pub fn park_core() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
