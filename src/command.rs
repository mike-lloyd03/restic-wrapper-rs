use restic_rs::Config;
use std::process::Command;

pub enum Location {
    Local,
    Remote,
}

fn run(mut cmd: Command) {
    cmd.spawn().unwrap().wait().unwrap();
}

pub fn backup(config: &Config) {
    let repo_name = &config.backup.repo_name;
    let repo = config.repos.get(repo_name).unwrap();

    let mut cmd = Command::new("restic");
    cmd.arg("backup")
        .args(["--repo", &repo.local_path])
        .args(["--password-file", &repo.local_pw_file])
        .arg("--exclude-caches");

    if let Some(excludes) = config.backup.exclude.as_ref() {
        for e in excludes {
            cmd.args(["--exclude", e]);
        }
    };

    config.backup.include.iter().for_each(|inc| {
        cmd.arg(inc);
    });

    run(cmd);
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
    let mut cmd = Command::new("restic");
    cmd.arg("check")
        .args(["--repo", repo_path])
        .args(["--password-file", pw_file]);

    run(cmd);
}

pub fn copy_to_remote(config: &Config, repo_name: String) {
    let repo = &config.repos.get(&repo_name).unwrap();
    let local_path = &repo.local_path;
    let local_pw_file = &repo.local_pw_file;
    let remote_repo_path = &repo.remote_path;
    let remote_pw_file = &repo.remote_pw_file;
    let mut cmd = Command::new("restic");
    cmd.arg("copy")
        .args(["--repo", local_path])
        .args(["--password-file", local_pw_file])
        .args(["--repo2", remote_repo_path])
        .args(["--password-file2", remote_pw_file]);

    run(cmd);
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

    let mut cmd = Command::new("restic");
    cmd.arg("forget")
        .args(["--repo", repo_path])
        .args(["--password-file", pw_file])
        .arg("--prune");

    if let Some(t) = &config.forget.keep_yearly {
        cmd.args(["--keep_yearly", &t.to_string()]);
    }
    if let Some(t) = &config.forget.keep_monthly {
        cmd.args(["--keep_monthly", &t.to_string()]);
    }
    if let Some(t) = &config.forget.keep_weekly {
        cmd.args(["--keep_weekly", &t.to_string()]);
    }
    if let Some(t) = &config.forget.keep_daily {
        cmd.args(["--keep_daily", &t.to_string()]);
    }

    run(cmd);
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
    let mut cmd = Command::new("restic");
    cmd.arg("snapshots")
        .args(["--repo", repo_path])
        .args(["--password-file", pw_file]);

    run(cmd);
}
