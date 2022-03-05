use clap::{Parser, Subcommand};
pub mod command;
use self::command::*;
use restic_rs::load_config;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Suppress standard output
    #[clap(short, long)]
    quiet: bool,

    /// Alternate configuration file to use
    #[clap(short, long, value_name = "FILE", default_value = "repos.yaml")]
    config_file: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a backup job now
    Backup,
    /// Check the condition of all configured repos
    Check,
    /// Copy a local repo to its remote counterpart
    CopyToRemote,
    /// Not implemented
    Mount,
    /// Not implemented
    Prune,
    /// Displays all snapshots available in the local and remote repos
    Snapshots,
    /// Not implemented
    Unlock,
}

fn main() {
    let cli = Cli::parse();
    let config = load_config(vec![&cli.config_file]).unwrap();

    match &cli.command {
        Commands::Backup => {
            backup(&config);
            forget(&config, config.backup.repo_name.clone(), Location::Local);
        }
        Commands::Check => {
            for repo in &config.repos {
                let repo_name = repo.0.to_owned();
                println!("\n-------- Checking {} local repo ----------", &repo_name);
                check(&config, repo_name.clone(), Location::Local);
                println!("\n-------- Checking {} remote repo ----------", &repo_name);
                check(&config, repo_name, Location::Remote)
            }
        }
        Commands::CopyToRemote => {
            for repo in &config.repos {
                let repo_name = repo.0.to_owned();
                copy_to_remote(&config, repo_name.clone());
                forget(&config, repo_name.clone(), Location::Remote);
            }
        }
        Commands::Mount => unimplemented!(),
        Commands::Prune => unimplemented!(),
        Commands::Snapshots => {
            for repo in &config.repos {
                let repo_name = repo.0.to_owned();
                println!("-------- {} local repo ----------", repo_name);
                snapshots(&config, config.backup.repo_name.clone(), Location::Local);
                println!("-------- {} remote repo ----------", repo_name);
                snapshots(&config, config.backup.repo_name.clone(), Location::Remote);
            }
        }
        Commands::Unlock => unimplemented!(),
    };
}

// fn output(res: Result) {}
