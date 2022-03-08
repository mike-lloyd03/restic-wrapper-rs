use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub repos: HashMap<String, Repo>,
    pub backup: Backup,
    pub copy: Option<Copy>,
    pub forget: Forget,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub path: String,
    pub pw_file: String,
}

#[derive(Debug, Deserialize)]
pub struct Backup {
    pub repo_name: String,
    pub exclude: Option<Vec<String>>,
    pub include: Vec<String>,
    pub pre_command: Option<String>,
    pub post_command: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Copy {
    pub pre_command: Option<String>,
    pub post_command: Option<String>,
    pub pairs: Vec<CopyPair>,
}

#[derive(Debug, Deserialize)]
pub struct CopyPair {
    pub src: String,
    pub dest: String,
}

#[derive(Debug, Deserialize)]
pub struct Forget {
    pub keep_daily: Option<u32>,
    pub keep_weekly: Option<u32>,
    pub keep_monthly: Option<u32>,
    pub keep_yearly: Option<u32>,
}

/// Loads configuration information from a yaml file. Paths are provided as a `Vec<&str>` of
/// locations where the configuration file can be found. The first match returned will be used.
///
/// # Examples
///
/// ```
/// use restic_rs::load_config;
///
/// load_config(vec!["/etc/program/config.yaml", "~/.config/config.yaml"])
/// ```
///
/// # Errors
///
/// This function will return an error if no configuration file can be found or if the located
/// configuration file cannot be deserialized.
pub fn load_config(paths: Vec<&str>) -> Result<Config> {
    use std::fs::File;
    use std::path::Path;

    let f: File;

    for path in paths {
        if Path::new(path).exists() {
            f = File::open(path)?;
            let config: Config = serde_yaml::from_reader(f)?;
            return validate_config(config);
        }
    }
    bail!("No configuration file was found.")
}

fn validate_config(config: Config) -> Result<Config> {
    let repos = &config.repos;
    let check_repo = |repo| -> bool { repos.get(repo).is_some() };

    // Check backup repo name
    if !check_repo(&config.backup.repo_name) {
        bail!("The repo defined for backup was not found in the repo map")
    };

    // Check src and dest repos of copy vec
    if let Some(copy) = &config.copy {
        for c in &copy.pairs {
            if !check_repo(&c.src) {
                bail!(
                    "The copy source \"{}\" was not found in the repo map.",
                    &c.src
                )
            }
            if !check_repo(&c.dest) {
                bail!(
                    "The copy destination \"{}\" was not found in the repo map.",
                    &c.dest
                )
            }
        }
    }
    Ok(config)
}
