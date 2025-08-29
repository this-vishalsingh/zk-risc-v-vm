//! ELF file parsing

use crate::utils::VmError;
use std::path::Path;

/// ELF file loader
pub struct ElfLoader {
    // TODO: Implement ELF loading
}

impl ElfLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load<P: AsRef<Path>>(&self, _path: P) -> Result<ElfFile, VmError> {
        // TODO: Implement ELF loading using goblin
        Err(VmError::ElfError("ELF loading not implemented".to_string()))
    }
}

/// Parsed ELF file representation
pub struct ElfFile {
    pub entry_point: u32,
    pub segments: Vec<ElfSegment>,
}

/// ELF segment
pub struct ElfSegment {
    pub virtual_addr: u32,
    pub data: Vec<u8>,
}