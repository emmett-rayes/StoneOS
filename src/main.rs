#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

/// # Architecture Module
///
/// This module provides architecture-specific functionality and abstractions for the target CPU architecture.
/// It is specific to the target hardware platform.
mod arch;

/// # Boot Module
///
/// This module handles the initial boot process of the operating system. It is responsible
/// for setting up essential hardware, initializing the kernel environment, and transitioning
/// control from the bootloader to the kernel.
mod boot;

/// # Board Support Package (BSP) module.
///
/// This module initializes and configures hardware peripherals, and provides hardware-specific functionality.
/// It is specific to the target hardware platform.
mod bsp;

/// # Console Module
///
mod console;

/// # CPU Module
///
/// This module provides general, i.e. not platform-specific, CPU-related abstractions.
mod cpu;

/// # Kernel Module
///
/// This is the core component of the operating system, responsible for essentials like task and memory management.
mod kernel;

/// # Memory Module
///
/// This module encapsulates memory-related functionality and implementations.
mod memory;

/// # Panic Module
///
/// This module defines a custom panic handler for operating system.
mod panic;

/// # Print Module
///
mod print;