//! Simple example demonstrating the ZK RISC-V VM

use zk_risc_v_vm::{VirtualMachine, VmError};
use zk_risc_v_vm::risc_v::RegisterIndex;

fn main() -> Result<(), VmError> {
    println!("ZK RISC-V Virtual Machine Example");
    println!("==================================");

    let mut vm = VirtualMachine::new();

    // Create a simple RISC-V program:
    // 1. Load 10 into x1
    // 2. Load 32 into x2  
    // 3. Add x1 + x2 and store in x3
    // 4. Exit with the result in x10
    
    let program = vec![
        0x00a00093u32, // addi x1, x0, 10
        0x02000113u32, // addi x2, x0, 32
        0x002081b3u32, // add x3, x1, x2
        0x00018513u32, // addi x10, x3, 0 (move x3 to x10 for exit code)
        0x05d00893u32, // addi x17, x0, 93 (exit syscall number)
        0x00000073u32, // ecall
    ];

    println!("Loading program into memory...");
    
    // Load program into memory starting at address 0x1000
    for (i, &instruction) in program.iter().enumerate() {
        vm.memory_mut().store_word(0x1000 + (i as u32) * 4, instruction)?;
    }

    // Set the program counter to start of program
    vm.set_pc(0x1000);

    println!("Initial state:");
    print_vm_state(&vm);

    println!("\nExecuting program...");
    
    // Execute the program
    vm.execute()?;

    println!("\nFinal state:");
    print_vm_state(&vm);
    
    let result = vm.cpu_state().read_register(RegisterIndex::X10);
    println!("\nProgram completed!");
    println!("Result (x10): {}", result);
    println!("Expected: 42 (10 + 32)");

    Ok(())
}

fn print_vm_state(vm: &VirtualMachine) {
    let stats = vm.get_stats();
    println!("  PC: 0x{:08x}", stats.pc);
    println!("  Cycles: {}", stats.cycles);
    println!("  Registers:");
    
    // Print some key registers
    for i in 0..8 {
        let reg = zk_risc_v_vm::risc_v::RegisterIndex::from_u32(i).unwrap();
        let value = vm.cpu_state().read_register(reg);
        println!("    x{}: 0x{:08x} ({})", i, value, value);
    }
}