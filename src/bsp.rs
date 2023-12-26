#[cfg(feature = "bsp_rpi4")]
pub mod rpi4;

#[cfg(feature = "bsp_rpi4")]
pub use crate::bsp::rpi4::RPi4 as Bsp;
