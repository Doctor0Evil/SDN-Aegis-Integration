use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aegis")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    BlueprintVerify,
    QpuValidateShard { shard: String },
    SnapshotHash,
    Allocations,
    SimRunScenario { name: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::BlueprintVerify => { println!("blueprint-verify: OK (stub)"); }
        Commands::QpuValidateShard { shard } => { println!("qpu-validate-shard: {} (stub)", shard); }
        Commands::SnapshotHash => { println!("snapshot-hash: (stub)"); }
        Commands::Allocations => { println!("allocations: (stub)"); }
        Commands::SimRunScenario { name } => { println!("sim run: {} (stub)", name); }
    }
}
