use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use zk_risc_v_vm::{VirtualMachine, Result};

#[derive(Parser)]
#[command(name = "zkvm")]
#[command(about = "Zero-Knowledge RISC-V Virtual Machine")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a RISC-V program and generate a proof
    Execute {
        /// Path to the ELF file to execute
        #[arg(short, long)]
        file: PathBuf,
        /// Generate zero-knowledge proof
        #[arg(short, long)]
        prove: bool,
        /// Output file for the proof
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Verify a previously generated proof
    Verify {
        /// Path to the proof file
        #[arg(short, long)]
        proof: PathBuf,
        /// Path to the verification key
        #[arg(short, long)]
        vkey: PathBuf,
    },
    /// Setup trusted parameters for the proof system
    Setup {
        /// Output directory for setup files
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Execute { file, prove, output } => {
            info!("Executing RISC-V program: {:?}", file);
            
            let mut vm = VirtualMachine::new();
            vm.load_elf(&file)?;
            
            if prove {
                info!("Generating zero-knowledge proof...");
                let proof = vm.execute_with_proof()?;
                
                if let Some(output_path) = output {
                    std::fs::write(&output_path, serde_json::to_string(&proof)?)?;
                    info!("Proof written to: {:?}", output_path);
                } else {
                    println!("{}", serde_json::to_string_pretty(&proof)?);
                }
            } else {
                vm.execute()?;
                info!("Program executed successfully");
            }
        }
        
        Commands::Verify { proof, vkey } => {
            info!("Verifying proof: {:?}", proof);
            // TODO: Implement proof verification
            println!("Proof verification not yet implemented");
        }
        
        Commands::Setup { output } => {
            info!("Setting up trusted parameters in: {:?}", output);
            // TODO: Implement trusted setup
            println!("Trusted setup not yet implemented");
        }
    }

    Ok(())
}