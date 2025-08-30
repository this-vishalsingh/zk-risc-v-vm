//! Test for actual instruction execution

use zk_risc_v_vm::{VirtualMachine, VmError};
use zk_risc_v_vm::risc_v::{RegisterIndex, Decoder, Instruction};
use zk_risc_v_vm::vm::Executor;

#[test]
fn test_instruction_decoding() -> Result<(), VmError> {
    // Test decoding a simple ADDI instruction: addi x1, x0, 42
    // Encoding: 0x02a00093
    let instruction_word = 0x02a00093u32;
    
    let decoded = Decoder::decode(instruction_word)?;
    
    match decoded {
        Instruction::Addi { rd, rs1, imm } => {
            assert_eq!(rd as u8, 1); // x1
            assert_eq!(rs1 as u8, 0); // x0
            assert_eq!(imm, 42);
        },
        _ => panic!("Expected ADDI instruction"),
    }
    
    Ok(())
}

#[test]
fn test_addi_execution() -> Result<(), VmError> {
    let mut vm = VirtualMachine::new();
    
    // Load ADDI instruction: addi x1, x0, 42
    let instruction_word = 0x02a00093u32;
    vm.memory_mut().store_word(0x1000, instruction_word)?;
    
    // Load an exit program after the ADDI
    vm.memory_mut().store_word(0x1004, 0x05d00893u32)?; // addi x17, x0, 93
    vm.memory_mut().store_word(0x1008, 0x00000513u32)?; // addi x10, x0, 0  
    vm.memory_mut().store_word(0x100c, 0x00000073u32)?; // ecall
    
    vm.set_pc(0x1000);
    
    // Execute the program
    vm.execute()?;
    
    // Check that x1 now contains 42
    assert_eq!(vm.cpu_state().read_register(RegisterIndex::X1), 42);
    
    Ok(())
}

#[test]
fn test_program_with_exit() -> Result<(), VmError> {
    let mut vm = VirtualMachine::new();
    
    // Create a simple program that exits
    // addi x17, x0, 93    # Load syscall number for exit (93) into x17
    // addi x10, x0, 0     # Load exit code 0 into x10 (a0)
    // ecall               # System call
    
    let program = vec![
        0x05d00893u32, // addi x17, x0, 93
        0x00000513u32, // addi x10, x0, 0
        0x00000073u32, // ecall
    ];
    
    // Load program into memory
    for (i, &instruction) in program.iter().enumerate() {
        vm.memory_mut().store_word(0x1000 + (i as u32) * 4, instruction)?;
    }
    
    vm.set_pc(0x1000);
    
    // Execute the program
    let result = vm.execute();
    
    // Should complete without error (exit syscall stops execution)
    assert!(result.is_ok());
    
    Ok(())
}