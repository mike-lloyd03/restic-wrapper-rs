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
    Check {
        /// The repository to check
        repo: Option<String>,
    },
    /// Copy all configured repo pairs
    CopyAll,
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
    /// Displays all snapshots available
    Snapshots {
        /// The repository to get snapshots from (default=all)
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

    match &app.args.command {
        Command::Init => init(&app),

        Command::Backup => {
            let repo_name = &app.config.backup.repo_name;
            backup(&app);
            forget(&app, repo_name.to_string());
        }

        Command::Check { repo } => {
            for r in &app.config.repos {
                let repo_name = r.0.to_owned();

                if let Some(repo_arg) = repo {
                    if &repo_name != repo_arg {
                        continue;
                    }
                }

                if !app.args.quiet {
                    println!("\n-------- Checking {} local repo ----------", &repo_name);
                }
                check(&app, repo_name.clone());
                if !app.args.quiet {
                    println!("\n-------- Checking {} remote repo ----------", &repo_name);
                }
                check(&app, repo_name)
            }
        }

        Command::CopyAll => {
            let copy_pairs = match &app.config.copy {
                Some(c) => c,
                None => {
                    eprintln!("No copy pairs are defined in the configuration file.");
                    exit(1);
                }
            };

            for c in copy_pairs {
                if !app.args.quiet {
                    println!("\n-------- Copying {} to {} ----------", &c.src, &c.dest);
                }

                copy(&app, c.src.to_string(), c.dest.to_string());
                forget(&app, c.dest.to_string());
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
                println!("-------- {} repo ----------", repo_name);
                snapshots(&app, repo_name.clone());
            }
        }

        Command::Unlock => unimplemented!(),
    };
}

// fn output(res: Result) {}
