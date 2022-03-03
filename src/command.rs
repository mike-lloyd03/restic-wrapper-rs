use restic_rs::Config;
use std::io::Error;
use std::process::{Command, Output};

/// Runs `restic backup`, excluding any defined directories from the configuration file.
///
/// # Examples
///
/// ```
/// use restic_rs::command::backup;
///
/// assert_eq!(backup(config), );
/// ```
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
pub fn backup(config: &Config) -> Result<Output, Error> {
    let repo_name = &config.backup.repo_name;
    let repo = config.repos.get(repo_name).unwrap();
    let mut cmd = Command::new("restic");
    cmd.arg("backup")
        .args(["--repo", repo_name])
        .args(["--password-file", &repo.local_pw_file])
        .arg("--exclude-caches");

    if let Some(excludes) = config.backup.exclude.as_ref() {
        for e in excludes {
            cmd.args(["--exclude", e]);
        }
    };

    let includes = &config.backup.include;
    for i in includes {
        cmd.arg(i);
    }

    cmd.output()
}

pub fn check(config: &Config) -> Result<Output, Error> {
    let repo_name = &config.backup.repo_name;
    let repo = config.repos.get(repo_name).unwrap();
    let mut cmd = Command::new("restic");
    cmd.arg("check")
        .args(["--repo", repo_name])
        .args(["--password-file", &repo.local_pw_file]);

    cmd.output()
}
