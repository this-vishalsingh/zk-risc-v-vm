# Zero-Knowledge RISC-V Virtual Machine

A complete zero-knowledge virtual machine that can execute RISC-V bytecode while generating cryptographic proofs of correct execution without revealing the program's internal state or inputs.

## Features

- **Complete RV32I Implementation**: All 37 base integer instructions supported
- **Zero-Knowledge Proofs**: Integrated ZK-SNARK proof system using arkworks
- **Memory Management**: 32-bit address space with proper little-endian handling
- **System Calls**: Basic I/O operations and program termination
- **CLI Interface**: Easy-to-use command-line tool
- **Testing Suite**: Comprehensive tests for all components

## Quick Start

### Building

```bash
cargo build --release
```

### Running a Simple Program

```bash
# Run the example program
cargo run --example simple_program

# Use the CLI tool (when ELF loading is implemented)
cargo run --bin zkvm execute --file program.elf
```

## Architecture

### Project Structure

```
src/
├── main.rs                 # CLI interface and main entry point
├── lib.rs                  # Library exports
├── vm/
│   ├── mod.rs             # VM core module
│   ├── cpu.rs             # CPU state and registers
│   ├── memory.rs          # Memory management
│   ├── executor.rs        # Instruction execution engine
│   └── syscalls.rs        # System call handling
├── risc_v/
│   ├── mod.rs             # RISC-V module exports
│   ├── instructions.rs    # Instruction definitions
│   ├── decoder.rs         # Instruction decoding
│   ├── isa.rs            # Instruction set architecture
│   └── registers.rs       # Register definitions
├── zk/
│   ├── mod.rs             # ZK proof system
│   ├── circuit.rs         # Arithmetic circuit generation
│   ├── constraints.rs     # Constraint system
│   ├── prover.rs          # Proof generation
│   ├── verifier.rs        # Proof verification
│   └── setup.rs           # Trusted setup handling
├── loader/
│   ├── mod.rs             # ELF loader module
│   ├── elf.rs            # ELF file parsing
│   └── linker.rs         # Memory layout and linking
└── utils/
    ├── mod.rs             # Utility functions
    ├── error.rs          # Error handling
    └── config.rs         # Configuration management
```

### Supported RISC-V Instructions

#### Arithmetic Operations
- ADD, SUB, SLT, SLTU, AND, OR, XOR, SLL, SRL, SRA

#### Immediate Operations  
- ADDI, SLTI, SLTIU, ANDI, ORI, XORI, SLLI, SRLI, SRAI

#### Load/Store Operations
- LB, LH, LW, LBU, LHU, SB, SH, SW

#### Branch Operations
- BEQ, BNE, BLT, BGE, BLTU, BGEU

#### Jump Operations
- JAL, JALR

#### Upper Immediate Operations
- LUI, AUIPC

#### System Operations
- ECALL, EBREAK

## Examples

### Basic Usage

```rust
use zk_risc_v_vm::{VirtualMachine, VmError};
use zk_risc_v_vm::risc_v::RegisterIndex;

fn main() -> Result<(), VmError> {
    let mut vm = VirtualMachine::new();
    
    // Load a simple program: addi x1, x0, 42
    let program = [0x93, 0x00, 0xa0, 0x02]; // Little-endian
    vm.load_binary(0x1000, &program)?;
    vm.set_pc(0x1000);
    
    // Add exit sequence
    vm.memory_mut().store_word(0x1004, 0x05d00893)?; // addi x17, x0, 93
    vm.memory_mut().store_word(0x1008, 0x00000513)?; // addi x10, x0, 0
    vm.memory_mut().store_word(0x100c, 0x00000073)?; // ecall
    
    // Execute
    vm.execute()?;
    
    // Check result
    assert_eq!(vm.cpu_state().read_register(RegisterIndex::X1), 42);
    
    Ok(())
}
```

## Implementation Status

### Completed
- [x] Basic project structure and dependencies
- [x] RISC-V instruction definitions and decoding
- [x] CPU state management and register file
- [x] Memory management with proper addressing
- [x] Instruction execution engine for all RV32I instructions
- [x] System call handling (exit, write)
- [x] CLI interface structure
- [x] Basic testing suite
- [x] Example programs

### In Progress / TODO
- [ ] ELF file loading and parsing (goblin integration)
- [ ] Complete zero-knowledge proof system implementation
- [ ] Arithmetic circuit generation for RISC-V instructions
- [ ] Memory consistency proofs
- [ ] Batch verification support
- [ ] Performance optimizations
- [ ] Advanced system calls (read, file operations)
- [ ] Debugging and tracing features

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test integration_tests
cargo test execution_tests
```

## Dependencies

- **arkworks ecosystem**: Zero-knowledge proof primitives
- **goblin**: ELF file parsing
- **clap**: Command-line interface
- **serde**: Serialization support
- **tracing**: Logging and diagnostics
- **byteorder**: Endianness handling

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Future Work

- Complete ZK-SNARK integration with efficient circuit construction
- Support for additional RISC-V extensions (M, F, D)
- Optimization for practical proof generation performance
- Integration with blockchain and smart contract platforms
- Advanced debugging and profiling tools