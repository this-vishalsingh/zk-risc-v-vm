//! System call handling

use crate::utils::VmError;
use crate::vm::{CpuState, Memory};
use crate::risc_v::RegisterIndex;
use std::io::{self, Write};

/// System call numbers (following Linux RISC-V ABI)
#[derive(Debug, Clone, Copy)]
pub enum Syscall {
    Exit = 93,
    Write = 64,
    Read = 63,
}

impl Syscall {
    /// Convert a system call number to a Syscall enum
    pub fn from_number(num: u32) -> Option<Self> {
        match num {
            93 => Some(Self::Exit),
            64 => Some(Self::Write),
            63 => Some(Self::Read),
            _ => None,
        }
    }
}

/// System call handler
pub struct SyscallHandler;

impl SyscallHandler {
    /// Handle a system call
    pub fn handle_syscall(
        cpu: &mut CpuState,
        memory: &mut Memory,
    ) -> Result<bool, VmError> {
        // System call number is in register a7 (x17)
        let syscall_num = cpu.read_register(RegisterIndex::X17);
        
        // Arguments are in registers a0-a6 (x10-x16)
        let arg0 = cpu.read_register(RegisterIndex::X10);
        let arg1 = cpu.read_register(RegisterIndex::X11);
        let arg2 = cpu.read_register(RegisterIndex::X12);

        let syscall = Syscall::from_number(syscall_num)
            .ok_or_else(|| VmError::SyscallError(format!("Unknown syscall: {}", syscall_num)))?;

        match syscall {
            Syscall::Exit => {
                // Exit with code in a0
                tracing::info!("Program exited with code: {}", arg0);
                return Ok(true); // Signal VM to stop
            },

            Syscall::Write => {
                // Write system call: write(fd, buf, count)
                let fd = arg0;
                let buf_addr = arg1;
                let count = arg2;

                if fd == 1 || fd == 2 {
                    // stdout or stderr
                    let data = memory.load_bytes(buf_addr, count as usize)
                        .map_err(|_| VmError::SyscallError("Invalid buffer address".to_string()))?;
                    
                    if fd == 1 {
                        io::stdout().write_all(data)
                            .map_err(|e| VmError::SyscallError(format!("Write failed: {}", e)))?;
                        io::stdout().flush()
                            .map_err(|e| VmError::SyscallError(format!("Flush failed: {}", e)))?;
                    } else {
                        io::stderr().write_all(data)
                            .map_err(|e| VmError::SyscallError(format!("Write failed: {}", e)))?;
                        io::stderr().flush()
                            .map_err(|e| VmError::SyscallError(format!("Flush failed: {}", e)))?;
                    }

                    // Return number of bytes written in a0
                    cpu.write_register(RegisterIndex::X10, count);
                } else {
                    return Err(VmError::SyscallError(format!("Unsupported file descriptor: {}", fd)));
                }
            },

            Syscall::Read => {
                // Read system call: read(fd, buf, count)
                let fd = arg0;
                let _buf_addr = arg1;
                let _count = arg2;

                if fd == 0 {
                    // stdin - not implemented yet
                    cpu.write_register(RegisterIndex::X10, 0);
                } else {
                    return Err(VmError::SyscallError(format!("Unsupported file descriptor: {}", fd)));
                }
            },
        }

        Ok(false) // Continue execution
    }
}