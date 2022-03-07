use clap::{Parser, Subcommand};
pub mod command;
use self::command::*;
use restic_rs::{load_config, Config};
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Suppress standard output
    #[clap(short, long)]
    quiet: bool,

    /// Arguments to pass to restic
    #[clap(short, long)]
    args: Option<String>,

    /// Alternate configuration file to use
    #[clap(short, long, value_name = "FILE", default_value = "repos.yaml")]
    config_file: String,

    #[clap(subcommand)]
    command: Command,
}

pub struct App {
    args: Args,
    config: Config,
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
    CopyToRemote {
        /// The repository to copy to remote (default=all)
        repo: Option<String>,
    },
    /// Mount the repository at the specified location
    Mount {
        repo: String,
        // location: Location,
        mount_point: String,
    },
    /// Prune repositories
    Prune {
        /// The repository to prune
        repo: Option<String>,
    },
    /// Displays all snapshots available in the local and remote repos
    Snapshots {
        /// The repository to get snapshots from
        repo: Option<String>,
    },
    /// Not implemented
    Unlock,
}

fn main() {
    let args = Args::parse();
    let config = match load_config(vec![&args.config_file]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Unable to load the configuration file. {}", e);
            exit(1);
        }
    };
    let app = App { args, config };
    // let args = Args::parse();
    // let mut config = load_config(vec![&args.config_file]).unwrap();
    // config.quiet = args.quiet;

    // dotenv::from_path("/etc/storj/s3creds").unwrap();
    // println!("{:?}", std::env::var_os("AWS_ACCESS_KEY_ID"));

    match &app.args.command {
        Command::Init => init(&app),
        Command::Backup => {
            backup(&app);
            forget(&app, app.config.backup.repo_name.clone(), Location::Local);
        }
        Command::Check => {
            for repo in &app.config.repos {
                let repo_name = repo.0.to_owned();
                if !app.args.quiet {
                    println!("\n-------- Checking {} local repo ----------", &repo_name);
                }
                check(&app, repo_name.clone(), Location::Local);
                if !app.args.quiet {
                    println!("\n-------- Checking {} remote repo ----------", &repo_name);
                }
                check(&app, repo_name, Location::Remote)
            }
        }
        Command::CopyToRemote { repo } => {
            for r in &app.config.repos {
                let repo_name = r.0.to_owned();

                if let Some(repo_arg) = repo {
                    if &repo_name != repo_arg {
                        continue;
                    }
                }
                if !app.args.quiet {
                    println!(
                        "\n-------- Copying {} local repo to remote ----------",
                        &repo_name
                    );
                }

                copy_to_remote(&app, repo_name.clone());
                forget(&app, repo_name.clone(), Location::Remote);
            }
        }
        Command::Mount { repo, mount_point } => {
            mount(&app, repo.to_string(), mount_point.to_string())
        }
        Command::Prune { repo } => {
            // This needs to be cleaned up to specify "local" or "remote" repo
            for r in &app.config.repos {
                let repo_name = r.0.to_owned();

                if let Some(repo_arg) = repo {
                    if &repo_name != repo_arg {
                        continue;
                    }
                }
                if !app.args.quiet {
                    println!("\n-------- Pruning {} ----------", &repo_name);
                }

                prune(&app, repo_name.clone());
            }
        }
        Command::Snapshots { repo } => {
            for r in &app.config.repos {
                let repo_name = r.0.to_owned();
                if let Some(repo_arg) = repo {
                    if &repo_name != repo_arg {
                        continue;
                    }
                }
                println!("-------- {} local repo ----------", repo_name);
                snapshots(&app, repo_name.clone(), Location::Local);
                println!("-------- {} remote repo ----------", repo_name);
                snapshots(&app, repo_name.clone(), Location::Remote);
            }
        }
        Command::Unlock => unimplemented!(),
    };
}

// fn output(res: Result) {}
