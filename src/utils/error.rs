//! Error handling for the ZK RISC-V VM

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VmError {
    #[error("Invalid instruction: {0:08x}")]
    InvalidInstruction(u32),
    
    #[error("Memory access violation at address {address:08x}")]
    MemoryViolation { address: u32 },
    
    #[error("Invalid register index: {0}")]
    InvalidRegister(u32),
    
    #[error("ELF parsing error: {0}")]
    ElfError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Proof generation failed: {0}")]
    ProofError(String),
    
    #[error("Verification failed: {0}")]
    VerificationError(String),
    
    #[error("System call error: {0}")]
    SyscallError(String),
    
    #[error("VM execution error: {0}")]
    ExecutionError(String),
}