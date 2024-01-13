use core::hint::spin_loop;

use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};

use crate::memory::address::PhysicalAddress;
use crate::memory::deref::DerefWrapper;

register_structs! {
    Registers {
        /// Data register
        (0x00 => dr: ReadWrite<u32, DR::Register>),
        /// Receive status register/error clear register (not used)
        (0x04 => _rsrecr: ReadWrite<u32>),
        /// Reserved
        (0x08 => _reserved_0),
        /// Flag register
        (0x18 => fr:  ReadOnly<u32, FR::Register>),
        /// Reserved
        (0x1C => _reserved_1),
        /// IrDA register (not used)
        (0x20 => _ilpr: ReadWrite<u32>),
        /// Integer Baud rate divisor
        (0x24 => ibrd: ReadWrite<u32, IBRD::Register>),
        /// Fractional Baud rate divisor
        (0x28 => fbrd: ReadWrite<u32, FBRD::Register>),
        /// Line control register
        (0x2c => lcrh: ReadWrite<u32, LCRH::Register>),
        /// Control register
        (0x30 => cr: ReadWrite<u32, CR::Register>),
        /// Interrupt FIFO level select register (not used)
        (0x34 => _ifls: ReadWrite<u32>),
        /// Interrupt mask set clear register (not used)
        (0x38 => _imsc: ReadWrite<u32>),
        /// Raw interrupt status register
        (0x3C => _ris: ReadOnly<u32>),
        /// Masked interrupt status register
        (0x40 => _mis: ReadOnly<u32>),
        /// Interrupt clear register
        (0x44 => icr: WriteOnly<u32, ICR::Register>),
        /// DMA control register (not used)
        (0x48 => _dmacr: ReadWrite<u32>),
        /// Reserved
        (0x4c => _reserved_2),
        /// Test control register (not used)
        (0x80 => _itcr: ReadWrite<u32>),
        /// Integration test input register√ (not used)
        (0x84 => _itip: ReadWrite<u32>),
        /// Integration test output register (not used)
        (0x88 => _itop: ReadWrite<u32>),
        /// Test data register (not used)
        (0x8c => _tdr: ReadWrite<u32>),
        (0x90 => @END),
    }
}

register_bitfields![
    u32, // Register width

    /// Data Register
    DR [
        OE   OFFSET(11) NUMBITS(1)  [],  // Overrun error
        BE   OFFSET(10) NUMBITS(1)  [],  // Break error
        PE   OFFSET(9)  NUMBITS(1)  [],  // Parity error
        FE   OFFSET(8)  NUMBITS(1)  [],  // Framing error
        DATA OFFSET(0)  NUMBITS(8)  [],  // read/write data character
    ],
    /// Flag Register
    FR [
        TXFE OFFSET(7) NUMBITS(1) [],  // Transmit FIFO empty
        RXFF OFFSET(6) NUMBITS(1) [],  // Receive FIFO full
        TXFF OFFSET(5) NUMBITS(1) [],  // Transmit FIFO full
        RXFE OFFSET(4) NUMBITS(1) [],  // Receive FIFO empty
        BUSY OFFSET(3) NUMBITS(1) [],  // UART busy
    ],
    /// Integer Baud rate divisor
    IBRD [
        IBRD OFFSET(0) NUMBITS(16) [],  // Integer Baud rate divisor
    ],
    /// Fractional Baud rate divisor
    FBRD [
        FBRD OFFSET(0) NUMBITS(6) [],  // Fractional Baud rate divisor
    ],
    /// Line control register
    LCRH [
        SPS  OFFSET(7) NUMBITS(1) [],  // Stick parity select
        WLEN OFFSET(5) NUMBITS(2) [    // Word length
            FiveBit  = 0b00,
            SixBit   = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],
        FEN  OFFSET(4) NUMBITS(1) [],  // Enable FIFOs
        STP2 OFFSET(3) NUMBITS(1) [],  // Two stop bits select
        EPS  OFFSET(2) NUMBITS(1) [],  // Even parity select
        PEN  OFFSET(1) NUMBITS(1) [],  // Parity enable
        BRK  OFFSET(0) NUMBITS(1) [],  // Send break
    ],
    /// Control register
    CR [
        CTSEN  OFFSET(15) NUMBITS(1) [],  // CTS hardware control flow enable
        RTSEN  OFFSET(14) NUMBITS(1) [],  // RTS hardware flow control enable
        RTS    OFFSET(11) NUMBITS(1) [],  // Request to send
        RXE    OFFSET(9)  NUMBITS(1) [],  // Receive enable
        TXE    OFFSET(8)  NUMBITS(1) [],  // Transmit enable
        LBE    OFFSET(7)  NUMBITS(1) [],  // Loopback enable
        UARTEN OFFSET(0)  NUMBITS(1) [],  // UART enable
    ],
    /// Interrupt clear register
    ICR [
        OEIC   OFFSET(7) NUMBITS(1) [],  // Overrun error interrupt clear
        BEIC   OFFSET(7) NUMBITS(1) [],  // Break error interrupt clear
        PEIC   OFFSET(7) NUMBITS(1) [],  // Parity error interrupt clear
        FEIC   OFFSET(7) NUMBITS(1) [],  // Framing error interrupt clear
        RTIC   OFFSET(6) NUMBITS(1) [],  // Receive timeout interrupt clear
        TXIC   OFFSET(5) NUMBITS(1) [],  // Receive interrupt clear
        RXIC   OFFSET(4) NUMBITS(1) [],  // Receive interrupt clear
        CTSMIC OFFSET(1) NUMBITS(1) [],  // Modem interrupt clear
    ],
];

pub struct Pl011 {
    registers: DerefWrapper<Registers>,
}

impl Pl011 {
    pub fn new(mmio_base: PhysicalAddress) -> Pl011 {
        Pl011 {
            registers: DerefWrapper::new(mmio_base),
        }
    }

    /// Initialize UART in 8 data bits - no parity - 1 stop bit mode.
    /// We only write the fields that differ from the default.
    pub fn init(&mut self, clock_speed: usize, baud_rate: usize) {
        // Disable UART
        self.registers.cr.modify(CR::UARTEN::CLEAR);

        // Clear pending interrupts
        self.registers.icr.set(0);

        // Set Baud rate
        let divisor = clock_speed * 64 / 16 / baud_rate;
        let integral = divisor / 64;
        let fractional = divisor % 64;
        self.registers.ibrd.write(IBRD::IBRD.val(integral as u32));
        self.registers.fbrd.write(FBRD::FBRD.val(fractional as u32));

        // Set word length and enable FIFO
        self.registers
            .lcrh
            .modify(LCRH::WLEN::EightBit + LCRH::FEN::SET);

        // Enable UART
        self.registers.cr.modify(CR::UARTEN::SET);
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.registers.fr.matches_all(FR::RXFE::SET) {
            while self.registers.fr.matches_all(FR::RXFE::SET) {
                spin_loop();
            }
        }

        self.registers.dr.get() as u8
    }

    pub fn write_byte(&mut self, byte: u8) {
        while self.registers.fr.matches_all(FR::TXFF::SET) {
            spin_loop();
        }

        self.registers.dr.set(byte as u32);
    }
}
