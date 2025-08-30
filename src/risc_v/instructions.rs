//! RISC-V instruction definitions

use crate::risc_v::registers::RegisterIndex;

/// RISC-V RV32I instruction types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    // Arithmetic operations
    Add { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    Sub { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    Slt { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    Sltu { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    And { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    Or { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    Xor { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    Sll { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    Srl { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },
    Sra { rd: RegisterIndex, rs1: RegisterIndex, rs2: RegisterIndex },

    // Immediate arithmetic operations
    Addi { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Slti { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Sltiu { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Andi { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Ori { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Xori { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Slli { rd: RegisterIndex, rs1: RegisterIndex, shamt: u32 },
    Srli { rd: RegisterIndex, rs1: RegisterIndex, shamt: u32 },
    Srai { rd: RegisterIndex, rs1: RegisterIndex, shamt: u32 },

    // Load operations
    Lb { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Lh { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Lw { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Lbu { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },
    Lhu { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },

    // Store operations
    Sb { rs1: RegisterIndex, rs2: RegisterIndex, imm: i32 },
    Sh { rs1: RegisterIndex, rs2: RegisterIndex, imm: i32 },
    Sw { rs1: RegisterIndex, rs2: RegisterIndex, imm: i32 },

    // Branch operations
    Beq { rs1: RegisterIndex, rs2: RegisterIndex, imm: i32 },
    Bne { rs1: RegisterIndex, rs2: RegisterIndex, imm: i32 },
    Blt { rs1: RegisterIndex, rs2: RegisterIndex, imm: i32 },
    Bge { rs1: RegisterIndex, rs2: RegisterIndex, imm: i32 },
    Bltu { rs1: RegisterIndex, rs2: RegisterIndex, imm: i32 },
    Bgeu { rs1: RegisterIndex, rs2: RegisterIndex, imm: i32 },

    // Jump operations
    Jal { rd: RegisterIndex, imm: i32 },
    Jalr { rd: RegisterIndex, rs1: RegisterIndex, imm: i32 },

    // Upper immediate operations
    Lui { rd: RegisterIndex, imm: i32 },
    Auipc { rd: RegisterIndex, imm: i32 },

    // System operations
    Ecall,
    Ebreak,
}

/// Instruction format types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionFormat {
    R,    // Register format
    I,    // Immediate format
    S,    // Store format
    B,    // Branch format
    U,    // Upper immediate format
    J,    // Jump format
}

impl Instruction {
    /// Get the instruction format
    pub fn format(&self) -> InstructionFormat {
        match self {
            Self::Add { .. } | Self::Sub { .. } | Self::Slt { .. } | Self::Sltu { .. }
            | Self::And { .. } | Self::Or { .. } | Self::Xor { .. }
            | Self::Sll { .. } | Self::Srl { .. } | Self::Sra { .. } => InstructionFormat::R,

            Self::Addi { .. } | Self::Slti { .. } | Self::Sltiu { .. }
            | Self::Andi { .. } | Self::Ori { .. } | Self::Xori { .. }
            | Self::Slli { .. } | Self::Srli { .. } | Self::Srai { .. }
            | Self::Lb { .. } | Self::Lh { .. } | Self::Lw { .. }
            | Self::Lbu { .. } | Self::Lhu { .. }
            | Self::Jalr { .. } => InstructionFormat::I,

            Self::Sb { .. } | Self::Sh { .. } | Self::Sw { .. } => InstructionFormat::S,

            Self::Beq { .. } | Self::Bne { .. } | Self::Blt { .. }
            | Self::Bge { .. } | Self::Bltu { .. } | Self::Bgeu { .. } => InstructionFormat::B,

            Self::Lui { .. } | Self::Auipc { .. } => InstructionFormat::U,

            Self::Jal { .. } => InstructionFormat::J,

            Self::Ecall | Self::Ebreak => InstructionFormat::I,
        }
    }
}