//! Zero-Knowledge RISC-V Virtual Machine
//!
//! This library provides a complete zero-knowledge virtual machine that can execute
//! RISC-V bytecode while generating cryptographic proofs of correct execution.

pub mod risc_v;
pub mod vm;
pub mod zk;
pub mod loader;
pub mod utils;

pub use utils::error::VmError;
pub use vm::VirtualMachine;
pub use risc_v::Instruction;
pub use zk::Proof;

/// Result type used throughout the library
pub type Result<T> = std::result::Result<T, VmError>;