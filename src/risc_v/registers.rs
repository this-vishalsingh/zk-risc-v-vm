//! RISC-V register definitions

/// RISC-V register indices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegisterIndex {
    X0 = 0,   // Zero register (always 0)
    X1 = 1,   // Return address register
    X2 = 2,   // Stack pointer
    X3 = 3,   // Global pointer
    X4 = 4,   // Thread pointer
    X5 = 5,   // Temporary register
    X6 = 6,   // Temporary register
    X7 = 7,   // Temporary register
    X8 = 8,   // Saved register / Frame pointer
    X9 = 9,   // Saved register
    X10 = 10, // Function argument / return value
    X11 = 11, // Function argument / return value
    X12 = 12, // Function argument
    X13 = 13, // Function argument
    X14 = 14, // Function argument
    X15 = 15, // Function argument
    X16 = 16, // Function argument
    X17 = 17, // Function argument
    X18 = 18, // Saved register
    X19 = 19, // Saved register
    X20 = 20, // Saved register
    X21 = 21, // Saved register
    X22 = 22, // Saved register
    X23 = 23, // Saved register
    X24 = 24, // Saved register
    X25 = 25, // Saved register
    X26 = 26, // Saved register
    X27 = 27, // Saved register
    X28 = 28, // Temporary register
    X29 = 29, // Temporary register
    X30 = 30, // Temporary register
    X31 = 31, // Temporary register
}

impl RegisterIndex {
    /// Convert a u32 to a register index
    pub fn from_u32(val: u32) -> Option<Self> {
        if val <= 31 {
            Some(unsafe { std::mem::transmute(val as u8) })
        } else {
            None
        }
    }
}

/// Register file containing 32 general-purpose registers
#[derive(Debug, Clone)]
pub struct RegisterFile {
    registers: [u32; 32],
}

impl RegisterFile {
    /// Create a new register file with all registers initialized to 0
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
        }
    }

    /// Read a register value
    pub fn read(&self, index: RegisterIndex) -> u32 {
        match index {
            RegisterIndex::X0 => 0, // x0 is always 0
            _ => self.registers[index as usize],
        }
    }

    /// Write a value to a register
    pub fn write(&mut self, index: RegisterIndex, value: u32) {
        match index {
            RegisterIndex::X0 => {}, // x0 is always 0, writes are ignored
            _ => self.registers[index as usize] = value,
        }
    }

    /// Reset all registers to 0
    pub fn reset(&mut self) {
        self.registers = [0; 32];
    }
}

impl Default for RegisterFile {
    fn default() -> Self {
        Self::new()
    }
}