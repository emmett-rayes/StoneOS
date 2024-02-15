use crate::console::Console;
use crate::cpu::Cpu;

#[cfg(feature = "bsp_rpi4")]
use crate::bsp::rpi4::RPi4 as BspImpl;

#[cfg(feature = "bsp_rpi4")]
pub mod rpi4;

trait IBsp: Console + Cpu {}

impl IBsp for BspImpl {}

pub type Bsp = BspImpl;
