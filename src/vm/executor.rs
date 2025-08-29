//! Instruction execution engine

use crate::risc_v::{Instruction, Decoder};
use crate::vm::{CpuState, Memory, SyscallHandler};
use crate::utils::VmError;

/// Instruction executor
pub struct Executor;

impl Executor {
    /// Execute a single instruction
    pub fn execute_instruction(
        instruction: &Instruction,
        cpu: &mut CpuState,
        memory: &mut Memory,
    ) -> Result<bool, VmError> {
        match instruction {
            // Arithmetic operations
            Instruction::Add { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                cpu.write_register(*rd, val1.wrapping_add(val2));
            },

            Instruction::Sub { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                cpu.write_register(*rd, val1.wrapping_sub(val2));
            },

            Instruction::Slt { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1) as i32;
                let val2 = cpu.read_register(*rs2) as i32;
                cpu.write_register(*rd, if val1 < val2 { 1 } else { 0 });
            },

            Instruction::Sltu { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                cpu.write_register(*rd, if val1 < val2 { 1 } else { 0 });
            },

            Instruction::And { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                cpu.write_register(*rd, val1 & val2);
            },

            Instruction::Or { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                cpu.write_register(*rd, val1 | val2);
            },

            Instruction::Xor { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                cpu.write_register(*rd, val1 ^ val2);
            },

            Instruction::Sll { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1);
                let shift = cpu.read_register(*rs2) & 0x1f;
                cpu.write_register(*rd, val1 << shift);
            },

            Instruction::Srl { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1);
                let shift = cpu.read_register(*rs2) & 0x1f;
                cpu.write_register(*rd, val1 >> shift);
            },

            Instruction::Sra { rd, rs1, rs2 } => {
                let val1 = cpu.read_register(*rs1) as i32;
                let shift = cpu.read_register(*rs2) & 0x1f;
                cpu.write_register(*rd, (val1 >> shift) as u32);
            },

            // Immediate arithmetic operations
            Instruction::Addi { rd, rs1, imm } => {
                let val = cpu.read_register(*rs1);
                cpu.write_register(*rd, val.wrapping_add(*imm as u32));
            },

            Instruction::Slti { rd, rs1, imm } => {
                let val = cpu.read_register(*rs1) as i32;
                cpu.write_register(*rd, if val < *imm { 1 } else { 0 });
            },

            Instruction::Sltiu { rd, rs1, imm } => {
                let val = cpu.read_register(*rs1);
                cpu.write_register(*rd, if val < (*imm as u32) { 1 } else { 0 });
            },

            Instruction::Andi { rd, rs1, imm } => {
                let val = cpu.read_register(*rs1);
                cpu.write_register(*rd, val & (*imm as u32));
            },

            Instruction::Ori { rd, rs1, imm } => {
                let val = cpu.read_register(*rs1);
                cpu.write_register(*rd, val | (*imm as u32));
            },

            Instruction::Xori { rd, rs1, imm } => {
                let val = cpu.read_register(*rs1);
                cpu.write_register(*rd, val ^ (*imm as u32));
            },

            Instruction::Slli { rd, rs1, shamt } => {
                let val = cpu.read_register(*rs1);
                cpu.write_register(*rd, val << shamt);
            },

            Instruction::Srli { rd, rs1, shamt } => {
                let val = cpu.read_register(*rs1);
                cpu.write_register(*rd, val >> shamt);
            },

            Instruction::Srai { rd, rs1, shamt } => {
                let val = cpu.read_register(*rs1) as i32;
                cpu.write_register(*rd, (val >> shamt) as u32);
            },

            // Load operations
            Instruction::Lb { rd, rs1, imm } => {
                let addr = cpu.read_register(*rs1).wrapping_add(*imm as u32);
                let val = memory.load_byte(addr)? as i8 as i32 as u32;
                cpu.write_register(*rd, val);
            },

            Instruction::Lh { rd, rs1, imm } => {
                let addr = cpu.read_register(*rs1).wrapping_add(*imm as u32);
                let val = memory.load_halfword(addr)? as i16 as i32 as u32;
                cpu.write_register(*rd, val);
            },

            Instruction::Lw { rd, rs1, imm } => {
                let addr = cpu.read_register(*rs1).wrapping_add(*imm as u32);
                let val = memory.load_word(addr)?;
                cpu.write_register(*rd, val);
            },

            Instruction::Lbu { rd, rs1, imm } => {
                let addr = cpu.read_register(*rs1).wrapping_add(*imm as u32);
                let val = memory.load_byte(addr)? as u32;
                cpu.write_register(*rd, val);
            },

            Instruction::Lhu { rd, rs1, imm } => {
                let addr = cpu.read_register(*rs1).wrapping_add(*imm as u32);
                let val = memory.load_halfword(addr)? as u32;
                cpu.write_register(*rd, val);
            },

            // Store operations
            Instruction::Sb { rs1, rs2, imm } => {
                let addr = cpu.read_register(*rs1).wrapping_add(*imm as u32);
                let val = cpu.read_register(*rs2) as u8;
                memory.store_byte(addr, val)?;
            },

            Instruction::Sh { rs1, rs2, imm } => {
                let addr = cpu.read_register(*rs1).wrapping_add(*imm as u32);
                let val = cpu.read_register(*rs2) as u16;
                memory.store_halfword(addr, val)?;
            },

            Instruction::Sw { rs1, rs2, imm } => {
                let addr = cpu.read_register(*rs1).wrapping_add(*imm as u32);
                let val = cpu.read_register(*rs2);
                memory.store_word(addr, val)?;
            },

            // Branch operations
            Instruction::Beq { rs1, rs2, imm } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                if val1 == val2 {
                    cpu.set_pc(cpu.get_pc().wrapping_add(*imm as u32));
                    return Ok(false);
                }
            },

            Instruction::Bne { rs1, rs2, imm } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                if val1 != val2 {
                    cpu.set_pc(cpu.get_pc().wrapping_add(*imm as u32));
                    return Ok(false);
                }
            },

            Instruction::Blt { rs1, rs2, imm } => {
                let val1 = cpu.read_register(*rs1) as i32;
                let val2 = cpu.read_register(*rs2) as i32;
                if val1 < val2 {
                    cpu.set_pc(cpu.get_pc().wrapping_add(*imm as u32));
                    return Ok(false);
                }
            },

            Instruction::Bge { rs1, rs2, imm } => {
                let val1 = cpu.read_register(*rs1) as i32;
                let val2 = cpu.read_register(*rs2) as i32;
                if val1 >= val2 {
                    cpu.set_pc(cpu.get_pc().wrapping_add(*imm as u32));
                    return Ok(false);
                }
            },

            Instruction::Bltu { rs1, rs2, imm } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                if val1 < val2 {
                    cpu.set_pc(cpu.get_pc().wrapping_add(*imm as u32));
                    return Ok(false);
                }
            },

            Instruction::Bgeu { rs1, rs2, imm } => {
                let val1 = cpu.read_register(*rs1);
                let val2 = cpu.read_register(*rs2);
                if val1 >= val2 {
                    cpu.set_pc(cpu.get_pc().wrapping_add(*imm as u32));
                    return Ok(false);
                }
            },

            // Jump operations
            Instruction::Jal { rd, imm } => {
                cpu.write_register(*rd, cpu.get_pc() + 4);
                cpu.set_pc(cpu.get_pc().wrapping_add(*imm as u32));
                return Ok(false);
            },

            Instruction::Jalr { rd, rs1, imm } => {
                let target = cpu.read_register(*rs1).wrapping_add(*imm as u32) & !1;
                cpu.write_register(*rd, cpu.get_pc() + 4);
                cpu.set_pc(target);
                return Ok(false);
            },

            // Upper immediate operations
            Instruction::Lui { rd, imm } => {
                cpu.write_register(*rd, *imm as u32);
            },

            Instruction::Auipc { rd, imm } => {
                cpu.write_register(*rd, cpu.get_pc().wrapping_add(*imm as u32));
            },

            // System operations
            Instruction::Ecall => {
                return SyscallHandler::handle_syscall(cpu, memory);
            },

            Instruction::Ebreak => {
                return Err(VmError::ExecutionError("Breakpoint encountered".to_string()));
            },
        }

        // Normal execution continues - increment PC
        cpu.increment_pc();
        Ok(false)
    }

    /// Execute a single cycle: fetch, decode, execute
    pub fn execute_cycle(
        cpu: &mut CpuState,
        memory: &mut Memory,
    ) -> Result<bool, VmError> {
        // Fetch
        let instruction_word = memory.load_word(cpu.get_pc())?;
        
        // Decode
        let instruction = Decoder::decode(instruction_word)?;
        
        // Execute
        Self::execute_instruction(&instruction, cpu, memory)
    }
}