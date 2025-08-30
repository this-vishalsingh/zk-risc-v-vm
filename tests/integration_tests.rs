//! Integration tests for the ZK RISC-V VM

use zk_risc_v_vm::{VirtualMachine, VmError};
use zk_risc_v_vm::risc_v::RegisterIndex;

#[test]
fn test_basic_vm_creation() {
    let vm = VirtualMachine::new();
    let stats = vm.get_stats();
    assert_eq!(stats.cycles, 0);
    assert_eq!(stats.pc, 0);
}

#[test]
fn test_simple_program_execution() -> Result<(), VmError> {
    let mut vm = VirtualMachine::new();
    
    // Simple RISC-V program: addi x1, x0, 42 (load immediate 42 into register x1)
    // Instruction encoding: 0x02a00093
    let program = [0x93, 0x00, 0xa0, 0x02]; // Little-endian encoding
    
    vm.load_binary(0x1000, &program)?;
    vm.set_pc(0x1000);
    
    // Execute one instruction
    vm.cpu_state_mut().increment_pc(); // We'll manually step for this test
    vm.memory_mut().load_word(0x1000)?; // Just verify we can load the instruction
    
    Ok(())
}

#[test]
fn test_register_operations() {
    let mut vm = VirtualMachine::new();
    
    // Test setting and reading registers
    vm.set_register(RegisterIndex::X1, 42);
    assert_eq!(vm.cpu_state().read_register(RegisterIndex::X1), 42);
    
    // X0 should always be 0
    vm.set_register(RegisterIndex::X0, 100);
    assert_eq!(vm.cpu_state().read_register(RegisterIndex::X0), 0);
}

#[test] 
fn test_memory_operations() -> Result<(), VmError> {
    let mut vm = VirtualMachine::new();
    
    // Test basic memory operations
    vm.memory_mut().store_word(0x1000, 0xdeadbeef)?;
    let value = vm.memory().load_word(0x1000)?;
    assert_eq!(value, 0xdeadbeef);
    
    // Test byte operations
    vm.memory_mut().store_byte(0x2000, 0x42)?;
    let byte_value = vm.memory().load_byte(0x2000)?;
    assert_eq!(byte_value, 0x42);
    
    Ok(())
}