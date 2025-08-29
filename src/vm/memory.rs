//! Memory management for the RISC-V VM

use crate::utils::VmError;
use byteorder::{LittleEndian, ByteOrder};

/// Memory implementation with 32-bit address space
#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8>,
    size: usize,
}

impl Memory {
    /// Create a new memory instance with the specified size
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            size,
        }
    }

    /// Load a byte from memory
    pub fn load_byte(&self, addr: u32) -> Result<u8, VmError> {
        let addr = addr as usize;
        if addr >= self.size {
            return Err(VmError::MemoryViolation { address: addr as u32 });
        }
        Ok(self.data[addr])
    }

    /// Load a half-word (16-bit) from memory (little-endian)
    pub fn load_halfword(&self, addr: u32) -> Result<u16, VmError> {
        let addr = addr as usize;
        if addr + 1 >= self.size {
            return Err(VmError::MemoryViolation { address: addr as u32 });
        }
        Ok(LittleEndian::read_u16(&self.data[addr..addr + 2]))
    }

    /// Load a word (32-bit) from memory (little-endian)
    pub fn load_word(&self, addr: u32) -> Result<u32, VmError> {
        let addr = addr as usize;
        if addr + 3 >= self.size {
            return Err(VmError::MemoryViolation { address: addr as u32 });
        }
        Ok(LittleEndian::read_u32(&self.data[addr..addr + 4]))
    }

    /// Store a byte to memory
    pub fn store_byte(&mut self, addr: u32, value: u8) -> Result<(), VmError> {
        let addr = addr as usize;
        if addr >= self.size {
            return Err(VmError::MemoryViolation { address: addr as u32 });
        }
        self.data[addr] = value;
        Ok(())
    }

    /// Store a half-word (16-bit) to memory (little-endian)
    pub fn store_halfword(&mut self, addr: u32, value: u16) -> Result<(), VmError> {
        let addr = addr as usize;
        if addr + 1 >= self.size {
            return Err(VmError::MemoryViolation { address: addr as u32 });
        }
        LittleEndian::write_u16(&mut self.data[addr..addr + 2], value);
        Ok(())
    }

    /// Store a word (32-bit) to memory (little-endian)
    pub fn store_word(&mut self, addr: u32, value: u32) -> Result<(), VmError> {
        let addr = addr as usize;
        if addr + 3 >= self.size {
            return Err(VmError::MemoryViolation { address: addr as u32 });
        }
        LittleEndian::write_u32(&mut self.data[addr..addr + 4], value);
        Ok(())
    }

    /// Load raw bytes from memory
    pub fn load_bytes(&self, addr: u32, len: usize) -> Result<&[u8], VmError> {
        let addr = addr as usize;
        if addr + len > self.size {
            return Err(VmError::MemoryViolation { address: addr as u32 });
        }
        Ok(&self.data[addr..addr + len])
    }

    /// Store raw bytes to memory
    pub fn store_bytes(&mut self, addr: u32, data: &[u8]) -> Result<(), VmError> {
        let addr = addr as usize;
        if addr + data.len() > self.size {
            return Err(VmError::MemoryViolation { address: addr as u32 });
        }
        self.data[addr..addr + data.len()].copy_from_slice(data);
        Ok(())
    }

    /// Get the memory size
    pub fn size(&self) -> usize {
        self.size
    }

    /// Clear all memory
    pub fn clear(&mut self) {
        self.data.fill(0);
    }
}