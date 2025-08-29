//! Configuration management

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    /// Memory size in bytes (default 4MB)
    pub memory_size: usize,
    /// Enable zero-knowledge proof generation
    pub enable_proofs: bool,
    /// Maximum number of execution cycles
    pub max_cycles: u64,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            memory_size: 4 * 1024 * 1024, // 4MB
            enable_proofs: false,
            max_cycles: 1_000_000,
        }
    }
}