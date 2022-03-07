use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub repos: HashMap<String, Repo>,
    pub backup: Backup,
    pub forget: Forget,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub local_path: String,
    pub local_pw_file: String,
    pub remote_path: String,
    pub remote_pw_file: String,
}

#[derive(Debug, Deserialize)]
pub struct Backup {
    pub repo_name: String,
    pub exclude: Option<Vec<String>>,
    pub include: Vec<String>,
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
            return Ok(serde_yaml::from_reader(f)?);
        }
    }
    bail!("No configuration file was found")
}
