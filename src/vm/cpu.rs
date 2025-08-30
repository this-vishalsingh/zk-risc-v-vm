//! CPU state management

use crate::risc_v::{RegisterFile, RegisterIndex};

/// CPU state including registers and program counter
#[derive(Debug, Clone)]
pub struct CpuState {
    /// General-purpose registers
    pub registers: RegisterFile,
    /// Program counter
    pub pc: u32,
    /// Cycle counter
    pub cycles: u64,
}

impl CpuState {
    /// Create a new CPU state with default values
    pub fn new() -> Self {
        Self {
            registers: RegisterFile::new(),
            pc: 0,
            cycles: 0,
        }
    }

    /// Reset the CPU state
    pub fn reset(&mut self) {
        self.registers.reset();
        self.pc = 0;
        self.cycles = 0;
    }

    /// Read a register value
    pub fn read_register(&self, index: RegisterIndex) -> u32 {
        self.registers.read(index)
    }

    /// Write a value to a register
    pub fn write_register(&mut self, index: RegisterIndex, value: u32) {
        self.registers.write(index, value);
    }

    /// Increment the program counter by 4 (size of a RISC-V instruction)
    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(4);
        self.cycles += 1;
    }

    /// Set the program counter to a specific value
    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
        self.cycles += 1;
    }

    /// Get the current program counter
    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    /// Get the current cycle count
    pub fn get_cycles(&self) -> u64 {
        self.cycles
    }
}

impl Default for CpuState {
    fn default() -> Self {
        Self::new()
    }
}