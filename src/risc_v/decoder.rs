//! RISC-V instruction decoder

use crate::risc_v::{Instruction, RegisterIndex};
use crate::utils::VmError;

pub struct Decoder;

impl Decoder {
    /// Decode a 32-bit instruction word into an Instruction enum
    pub fn decode(instruction: u32) -> Result<Instruction, VmError> {
        let opcode = instruction & 0x7f;
        let rd = RegisterIndex::from_u32((instruction >> 7) & 0x1f)
            .ok_or(VmError::InvalidInstruction(instruction))?;
        let funct3 = (instruction >> 12) & 0x7;
        let rs1 = RegisterIndex::from_u32((instruction >> 15) & 0x1f)
            .ok_or(VmError::InvalidInstruction(instruction))?;
        let rs2 = RegisterIndex::from_u32((instruction >> 20) & 0x1f)
            .ok_or(VmError::InvalidInstruction(instruction))?;
        let funct7 = (instruction >> 25) & 0x7f;

        match opcode {
            // R-type instructions (OP)
            0x33 => {
                match (funct3, funct7) {
                    (0x0, 0x00) => Ok(Instruction::Add { rd, rs1, rs2 }),
                    (0x0, 0x20) => Ok(Instruction::Sub { rd, rs1, rs2 }),
                    (0x1, 0x00) => Ok(Instruction::Sll { rd, rs1, rs2 }),
                    (0x2, 0x00) => Ok(Instruction::Slt { rd, rs1, rs2 }),
                    (0x3, 0x00) => Ok(Instruction::Sltu { rd, rs1, rs2 }),
                    (0x4, 0x00) => Ok(Instruction::Xor { rd, rs1, rs2 }),
                    (0x5, 0x00) => Ok(Instruction::Srl { rd, rs1, rs2 }),
                    (0x5, 0x20) => Ok(Instruction::Sra { rd, rs1, rs2 }),
                    (0x6, 0x00) => Ok(Instruction::Or { rd, rs1, rs2 }),
                    (0x7, 0x00) => Ok(Instruction::And { rd, rs1, rs2 }),
                    _ => Err(VmError::InvalidInstruction(instruction)),
                }
            },

            // I-type instructions (OP-IMM)
            0x13 => {
                let imm = ((instruction as i32) >> 20) as i32;
                let shamt = (instruction >> 20) & 0x1f;
                
                match funct3 {
                    0x0 => Ok(Instruction::Addi { rd, rs1, imm }),
                    0x1 => Ok(Instruction::Slli { rd, rs1, shamt }),
                    0x2 => Ok(Instruction::Slti { rd, rs1, imm }),
                    0x3 => Ok(Instruction::Sltiu { rd, rs1, imm }),
                    0x4 => Ok(Instruction::Xori { rd, rs1, imm }),
                    0x5 => {
                        if (instruction >> 30) & 1 == 0 {
                            Ok(Instruction::Srli { rd, rs1, shamt })
                        } else {
                            Ok(Instruction::Srai { rd, rs1, shamt })
                        }
                    },
                    0x6 => Ok(Instruction::Ori { rd, rs1, imm }),
                    0x7 => Ok(Instruction::Andi { rd, rs1, imm }),
                    _ => Err(VmError::InvalidInstruction(instruction)),
                }
            },

            // Load instructions
            0x03 => {
                let imm = ((instruction as i32) >> 20) as i32;
                
                match funct3 {
                    0x0 => Ok(Instruction::Lb { rd, rs1, imm }),
                    0x1 => Ok(Instruction::Lh { rd, rs1, imm }),
                    0x2 => Ok(Instruction::Lw { rd, rs1, imm }),
                    0x4 => Ok(Instruction::Lbu { rd, rs1, imm }),
                    0x5 => Ok(Instruction::Lhu { rd, rs1, imm }),
                    _ => Err(VmError::InvalidInstruction(instruction)),
                }
            },

            // Store instructions
            0x23 => {
                let imm = (((instruction as i32) >> 25) << 5) | (((instruction >> 7) & 0x1f) as i32);
                
                match funct3 {
                    0x0 => Ok(Instruction::Sb { rs1, rs2, imm }),
                    0x1 => Ok(Instruction::Sh { rs1, rs2, imm }),
                    0x2 => Ok(Instruction::Sw { rs1, rs2, imm }),
                    _ => Err(VmError::InvalidInstruction(instruction)),
                }
            },

            // Branch instructions
            0x63 => {
                let imm = (((instruction as i32) >> 31) << 12) |
                         ((((instruction >> 7) & 1) as i32) << 11) |
                         ((((instruction >> 25) & 0x3f) as i32) << 5) |
                         ((((instruction >> 8) & 0xf) as i32) << 1);
                
                match funct3 {
                    0x0 => Ok(Instruction::Beq { rs1, rs2, imm }),
                    0x1 => Ok(Instruction::Bne { rs1, rs2, imm }),
                    0x4 => Ok(Instruction::Blt { rs1, rs2, imm }),
                    0x5 => Ok(Instruction::Bge { rs1, rs2, imm }),
                    0x6 => Ok(Instruction::Bltu { rs1, rs2, imm }),
                    0x7 => Ok(Instruction::Bgeu { rs1, rs2, imm }),
                    _ => Err(VmError::InvalidInstruction(instruction)),
                }
            },

            // JAL
            0x6f => {
                let imm = (((instruction as i32) >> 31) << 20) |
                         ((((instruction >> 12) & 0xff) as i32) << 12) |
                         ((((instruction >> 20) & 1) as i32) << 11) |
                         ((((instruction >> 21) & 0x3ff) as i32) << 1);
                
                Ok(Instruction::Jal { rd, imm })
            },

            // JALR
            0x67 => {
                let imm = ((instruction as i32) >> 20) as i32;
                
                if funct3 == 0x0 {
                    Ok(Instruction::Jalr { rd, rs1, imm })
                } else {
                    Err(VmError::InvalidInstruction(instruction))
                }
            },

            // LUI
            0x37 => {
                let imm = (instruction & 0xfffff000) as i32;
                Ok(Instruction::Lui { rd, imm })
            },

            // AUIPC
            0x17 => {
                let imm = (instruction & 0xfffff000) as i32;
                Ok(Instruction::Auipc { rd, imm })
            },

            // System instructions
            0x73 => {
                match funct3 {
                    0x0 => {
                        match instruction {
                            0x00000073 => Ok(Instruction::Ecall),
                            0x00100073 => Ok(Instruction::Ebreak),
                            _ => Err(VmError::InvalidInstruction(instruction)),
                        }
                    },
                    _ => Err(VmError::InvalidInstruction(instruction)),
                }
            },

            _ => Err(VmError::InvalidInstruction(instruction)),
        }
    }
}