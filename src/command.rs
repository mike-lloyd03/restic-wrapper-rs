use enum_utils::FromStr;
use restic_rs::Config;
use std::process::Command;

#[derive(Debug, FromStr)]
pub enum Location {
    Local,
    Remote,
}

struct Restic {
    cmd: Command,
}

impl Restic {
    fn new(subcommand: &str) -> Restic {
        let mut cmd = Command::new("restic");
        cmd.arg(subcommand);
        Restic { cmd }
    }

    /// Redirects stdout to /dev/null if `quiet = true`
    fn quiet(mut self, quiet: bool) -> Restic {
        if quiet {
            self.cmd.stdout(std::process::Stdio::null());
        }
        self
    }

    fn run(mut self) {
        self.cmd.spawn().unwrap().wait().unwrap();
    }
}

pub fn backup(config: &Config) {
    let repo_name = &config.backup.repo_name;
    let repo = config.repos.get(repo_name).unwrap();

    let mut restic = Restic::new("backup");
    restic
        .cmd
        .args(["--repo", &repo.local_path])
        .args(["--password-file", &repo.local_pw_file])
        .arg("--exclude-caches");

    // restic.quiet(config.quiet);

    if let Some(excludes) = config.backup.exclude.as_ref() {
        for e in excludes {
            restic.cmd.args(["--exclude", e]);
        }
    };

    config.backup.include.iter().for_each(|inc| {
        restic.cmd.arg(inc);
    });

    restic.quiet(config.quiet).run();
}

pub fn check(config: &Config, repo_name: String, location: Location) {
    let repo = &config.repos.get(&repo_name).unwrap();
    let repo_path: &str;
    let pw_file: &str;
    match location {
        Location::Local => {
            repo_path = &repo.local_path;
            pw_file = &repo.local_pw_file;
        }
        Location::Remote => {
            repo_path = &repo.remote_path;
            pw_file = &repo.remote_pw_file;
        }
    }
    let mut restic = Restic::new("check");
    restic
        .cmd
        .args(["--repo", repo_path])
        .args(["--password-file", pw_file]);

    restic.quiet(config.quiet).run();
}

pub fn copy_to_remote(config: &Config, repo_name: String) {
    let repo = &config.repos.get(&repo_name).unwrap();
    let local_path = &repo.local_path;
    let local_pw_file = &repo.local_pw_file;
    let remote_repo_path = &repo.remote_path;
    let remote_pw_file = &repo.remote_pw_file;

    let mut restic = Restic::new("copy");
    restic
        .cmd
        .args(["--repo", local_path])
        .args(["--password-file", local_pw_file])
        .args(["--repo2", remote_repo_path])
        .args(["--password-file2", remote_pw_file]);

    // restic.quiet(config.quiet);

    restic.quiet(config.quiet).run();
}

pub fn forget(config: &Config, repo_name: String, location: Location) {
    let repo = &config.repos.get(&repo_name).unwrap();
    let repo_path: &str;
    let pw_file: &str;
    match location {
        Location::Local => {
            repo_path = &repo.local_path;
            pw_file = &repo.local_pw_file;
        }
        Location::Remote => {
            repo_path = &repo.remote_path;
            pw_file = &repo.remote_pw_file;
        }
    }

    let mut restic = Restic::new("forget");
    restic
        .cmd
        .args(["--repo", repo_path])
        .args(["--password-file", pw_file])
        .arg("--prune");

    if let Some(t) = &config.forget.keep_yearly {
        restic.cmd.args(["--keep-yearly", &t.to_string()]);
    }
    if let Some(t) = &config.forget.keep_monthly {
        restic.cmd.args(["--keep-monthly", &t.to_string()]);
    }
    if let Some(t) = &config.forget.keep_weekly {
        restic.cmd.args(["--keep-weekly", &t.to_string()]);
    }
    if let Some(t) = &config.forget.keep_daily {
        restic.cmd.args(["--keep-daily", &t.to_string()]);
    }

    if config.quiet {
        restic.cmd.stdout(std::process::Stdio::null());
    }

    // restic.quiet(config.quiet);

    restic.quiet(config.quiet).run();
}

pub fn snapshots(config: &Config, repo_name: String, location: Location) {
    let repo = &config.repos.get(&repo_name).unwrap();
    let repo_path: &str;
    let pw_file: &str;
    match location {
        Location::Local => {
            repo_path = &repo.local_path;
            pw_file = &repo.local_pw_file;
        }
        Location::Remote => {
            repo_path = &repo.remote_path;
            pw_file = &repo.remote_pw_file;
        }
    }

    let mut restic = Restic::new("snapshots");
    restic
        .cmd
        .args(["--repo", repo_path])
        .args(["--password-file", pw_file]);

    restic.quiet(config.quiet).run();
}

pub fn mount(config: &Config, repo_name: String, mount_point: String) {
    let repo = &config.repos.get(&repo_name).unwrap();
    let repo_path = &repo.local_path;
    let pw_file = &repo.local_pw_file;

    let mut restic = Restic::new("mount");
    restic
        .cmd
        .arg(mount_point)
        .args(["--repo", repo_path])
        .args(["--password-file", pw_file]);

    restic.quiet(config.quiet).run();
}
