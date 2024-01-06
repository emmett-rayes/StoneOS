use crate::bsp::rpi4::RPi4;
use crate::console::{Console, Read, Write};

impl Console for RPi4 {
    fn console() -> impl Read + Write {
        todo!()
    }
}
