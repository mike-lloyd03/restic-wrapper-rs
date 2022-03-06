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
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize a new repo
    Init,
    /// Run a backup job now
    Backup,
    /// Check the condition of all configured repos
    Check,
    /// Copy a local repo to its remote counterpart
    CopyToRemote,
    /// Mount the repository at the specified location
    Mount {
        repo: String,
        // location: Location,
        mount_point: String,
    },
    /// Not implemented
    Prune,
    /// Displays all snapshots available in the local and remote repos
    Snapshots,
    /// Not implemented
    Unlock,
}

fn main() {
    let cli = Cli::parse();
    let mut config = load_config(vec![&cli.config_file]).unwrap();
    config.quiet = cli.quiet;

    // dotenv::from_path("/etc/storj/s3creds").unwrap();
    // println!("{:?}", std::env::var_os("AWS_ACCESS_KEY_ID"));

    match &cli.command {
        Command::Init => init(&config),
        Command::Backup => {
            backup(&config);
            forget(&config, config.backup.repo_name.clone(), Location::Local);
        }
        Command::Check => {
            for repo in &config.repos {
                let repo_name = repo.0.to_owned();
                if !config.quiet {
                    println!("\n-------- Checking {} local repo ----------", &repo_name);
                }
                check(&config, repo_name.clone(), Location::Local);
                if !config.quiet {
                    println!("\n-------- Checking {} remote repo ----------", &repo_name);
                }
                check(&config, repo_name, Location::Remote)
            }
        }
        Command::CopyToRemote => {
            for repo in &config.repos {
                let repo_name = repo.0.to_owned();
                copy_to_remote(&config, repo_name.clone());
                forget(&config, repo_name.clone(), Location::Remote);
            }
        }
        Command::Mount { repo, mount_point } => {
            mount(&config, repo.to_string(), mount_point.to_string())
        }
        Command::Prune => unimplemented!(),
        Command::Snapshots => {
            for repo in &config.repos {
                let repo_name = repo.0.to_owned();
                println!("-------- {} local repo ----------", repo_name);
                snapshots(&config, config.backup.repo_name.clone(), Location::Local);
                println!("-------- {} remote repo ----------", repo_name);
                snapshots(&config, config.backup.repo_name.clone(), Location::Remote);
            }
        }
        Command::Unlock => unimplemented!(),
    };
}

// fn output(res: Result) {}
