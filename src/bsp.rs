#[cfg(feature = "bsp_rpi4")]
pub use crate::bsp::rpi4::RPi4 as Bsp;

#[cfg(feature = "bsp_rpi4")]
pub mod rpi4;
