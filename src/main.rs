use clap::{Parser, Subcommand};
use std::process::exit;

pub mod command;
use self::command::*;
use restic_rs::load_config;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long)]
    quiet_flag: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Backup,
    Check,
    CopyToRemote,
    Mount,
    Prune,
    Snapshots,
    Unlock,
}

fn main() {
    let cli = Cli::parse();
    let config = load_config(vec!["repos.yaml"]).unwrap();

    let output = match &cli.command {
        Commands::Backup => backup(&config),
        Commands::Check => check(&config),
        Commands::CopyToRemote => unimplemented!(),
        Commands::Mount => unimplemented!(),
        Commands::Prune => unimplemented!(),
        Commands::Snapshots => unimplemented!(),
        Commands::Unlock => unimplemented!(),
    }
    .unwrap();

    if !&output.stdout.is_empty() && !cli.quiet_flag {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if !&output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
    exit(output.status.code().unwrap())
}

// fn output(res: Result) {}
