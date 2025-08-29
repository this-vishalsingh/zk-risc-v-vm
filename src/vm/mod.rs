//! Virtual Machine implementation

use crate::utils::{VmError, config::VmConfig};
use crate::zk::Proof;
use crate::risc_v::RegisterIndex;
use std::path::Path;

pub mod cpu;
pub mod memory;
pub mod executor;
pub mod syscalls;

pub use cpu::CpuState;
pub use memory::Memory;
pub use executor::Executor;
pub use syscalls::SyscallHandler;

/// Main Virtual Machine
#[derive(Debug)]
pub struct VirtualMachine {
    cpu: CpuState,
    memory: Memory,
    config: VmConfig,
}

impl VirtualMachine {
    /// Create a new virtual machine with default configuration
    pub fn new() -> Self {
        let config = VmConfig::default();
        Self::with_config(config)
    }

    /// Create a new virtual machine with the specified configuration
    pub fn with_config(config: VmConfig) -> Self {
        Self {
            cpu: CpuState::new(),
            memory: Memory::new(config.memory_size),
            config,
        }
    }

    /// Load an ELF file into memory
    pub fn load_elf<P: AsRef<Path>>(&mut self, _path: P) -> Result<(), VmError> {
        // For now, just return an error - ELF loading will be implemented later
        Err(VmError::ElfError("ELF loading not yet implemented".to_string()))
    }

    /// Load raw binary data into memory at the specified address
    pub fn load_binary(&mut self, addr: u32, data: &[u8]) -> Result<(), VmError> {
        self.memory.store_bytes(addr, data)?;
        Ok(())
    }

    /// Set the program counter to start execution
    pub fn set_pc(&mut self, pc: u32) {
        self.cpu.set_pc(pc);
    }

    /// Set initial register values
    pub fn set_register(&mut self, reg: RegisterIndex, value: u32) {
        self.cpu.write_register(reg, value);
    }

    /// Execute the program without proof generation
    pub fn execute(&mut self) -> Result<(), VmError> {
        loop {
            // Check cycle limit
            if self.cpu.get_cycles() >= self.config.max_cycles {
                return Err(VmError::ExecutionError("Maximum cycles exceeded".to_string()));
            }

            // Execute one cycle
            let should_stop = Executor::execute_cycle(&mut self.cpu, &mut self.memory)?;
            
            if should_stop {
                break;
            }
        }

        Ok(())
    }

    /// Execute the program with proof generation
    pub fn execute_with_proof(&mut self) -> Result<Proof, VmError> {
        if !self.config.enable_proofs {
            return Err(VmError::ProofError("Proof generation not enabled".to_string()));
        }

        // For now, execute without proof and return a dummy proof
        self.execute()?;
        
        // TODO: Implement actual proof generation
        Ok(Proof::dummy())
    }

    /// Get the current CPU state
    pub fn cpu_state(&self) -> &CpuState {
        &self.cpu
    }

    /// Get mutable reference to CPU state
    pub fn cpu_state_mut(&mut self) -> &mut CpuState {
        &mut self.cpu
    }

    /// Get the memory
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Get mutable reference to memory
    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    /// Reset the virtual machine
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.clear();
    }

    /// Get execution statistics
    pub fn get_stats(&self) -> VmStats {
        VmStats {
            cycles: self.cpu.get_cycles(),
            pc: self.cpu.get_pc(),
            memory_size: self.memory.size(),
        }
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// Virtual machine execution statistics
#[derive(Debug, Clone)]
pub struct VmStats {
    pub cycles: u64,
    pub pc: u32,
    pub memory_size: usize,
}