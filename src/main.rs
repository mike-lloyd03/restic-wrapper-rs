use clap::{Parser, Subcommand};
use std::process::exit;

mod command;
mod config;
mod snapshots;

use command::*;
use config::Config;
use snapshots::Snapshot;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Suppress standard output
    #[clap(short, long)]
    quiet: bool,

    /// Alternate configuration file to use
    #[clap(
        short,
        long,
        value_name = "FILE",
        default_value = "/etc/restic-rs/repos.yaml"
    )]
    config_file: String,

    /// Do not upload or write any data, just show what would be done
    #[clap(short = 'n', long)]
    dry_run: bool,

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
    /// List all configured repos
    List,
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
    /// Unlocks the specified repo
    Unlock {
        /// The repository to unlock
        repo: Option<String>,
    },
    /// Displays statistics about the configured repos
    Stats {
        /// The repository to calculate statistics for (required if more than one repo is defined
        /// in the config file.) (Optional)
        repo: Option<String>,

        /// The snapshot ID to calculate statistics for. If more than one repo is configured, a
        /// repo must be specified. (Optional)
        snapshot_id: Option<String>,

        /// Calculates the statistics for each snapshot in the repo
        #[clap(short, long)]
        iterate_over_snapshots: bool,
    },
    /// Forgets either a specified snapshot or forgets snapshots according to the configuration
    Forget {
        /// The repository from which snapshots will be deleted
        repo: Option<String>,

        /// The snapshot ID to forget
        snapshot_id: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    let config = Config::load(vec![&args.config_file]).unwrap_or_else(|e| {
        eprintln!("Unable to load the configuration file. {}", e);
        exit(1);
    });
    let app = App { args, config };

    if let Some(cmd) = &app.config.pre_command {
        command::run_cmd(cmd)
    }

    match &app.args.command {
        Command::Init => init(&app),

        Command::Backup => {
            let repo_name = &app.config.backup.repo_name;
            backup(&app);
            forget(&app, repo_name.to_string(), None);
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
                    println!("\n-------- Checking {} repo ----------", &repo_name);
                }
                check(&app, repo_name);
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

            for c in &copy_pairs.pairs {
                if !app.args.quiet {
                    println!("\n-------- Copying {} to {} ----------", &c.src, &c.dest);
                }

                copy(&app, c.src.to_string(), c.dest.to_string());
                forget(&app, c.dest.to_string(), None);
            }
        }

        Command::List => list(&app),

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

        Command::Unlock { repo } => {
            let repo_name = match_repo_name(&app, repo.to_owned());
            unlock(&app, repo_name);
        }

        Command::Stats {
            repo,
            snapshot_id,
            iterate_over_snapshots,
        } => {
            let repo_name = match_repo_name(&app, repo.to_owned());

            if *iterate_over_snapshots {
                let snaps_string = snapshots_json(&app, repo_name.clone());
                let snaps = Snapshot::from_string(snaps_string);

                for s in snaps {
                    println!("Snapshot id {}", s.short_id);
                    stats(&app, repo_name.clone(), Some(s.short_id));
                }
            } else {
                stats(&app, repo_name, snapshot_id.to_owned());
            }
        }
        Command::Forget { repo, snapshot_id } => {
            let repo_name = match_repo_name(&app, repo.to_owned());
            forget(&app, repo_name, snapshot_id.to_owned());
        }
    };

    if let Some(cmd) = &app.config.post_command {
        command::run_cmd(cmd)
    }
}

/// Checks to see if the given repo matches
fn match_repo_name(app: &App, repo: Option<String>) -> String {
    match repo {
        Some(repo) => repo,
        None => {
            if app.config.repos.len() == 1 {
                (&app.config.repos.iter().next().unwrap().0).to_string()
            } else {
                eprintln!(
                    "If more than one repo is defined, you must provide the name of the repo. Repo name must be one of:\n"
                );
                app.config.repos.iter().for_each(|r| eprintln!("{}", r.0));
                exit(1);
            }
        }
    }
}
