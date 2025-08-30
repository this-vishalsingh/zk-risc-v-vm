//! Zero-knowledge proof system

use serde::{Deserialize, Serialize};

pub mod circuit;
pub mod constraints;
pub mod prover;
pub mod verifier;
pub mod setup;

pub use prover::Prover;
pub use verifier::Verifier;

/// Zero-knowledge proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub data: Vec<u8>,
}

impl Proof {
    /// Create a dummy proof for testing
    pub fn dummy() -> Self {
        Self {
            data: vec![0u8; 32],
        }
    }
}

/// Proof system interface
pub trait ProofSystem {
    /// Generate a proof for the given execution trace
    fn generate_proof(&self, trace: &ExecutionTrace) -> Result<Proof, crate::utils::VmError>;
    
    /// Verify a proof
    fn verify_proof(&self, proof: &Proof, public_inputs: &[u8]) -> Result<bool, crate::utils::VmError>;
}

/// Execution trace for proof generation
#[derive(Debug, Clone)]
pub struct ExecutionTrace {
    pub instructions: Vec<u32>,
    pub register_states: Vec<[u32; 32]>,
    pub memory_accesses: Vec<MemoryAccess>,
    pub cycle_count: u64,
}

/// Memory access record
#[derive(Debug, Clone)]
pub struct MemoryAccess {
    pub address: u32,
    pub value: u32,
    pub is_write: bool,
    pub cycle: u64,
}