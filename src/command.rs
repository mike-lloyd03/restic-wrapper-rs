use crate::App;
use std::process::{exit, Command};

struct Restic {
    cmd: Command,
}

impl Restic {
    /// Creates a new Restic command
    fn new(subcommand: &str) -> Self {
        let mut cmd = Command::new("/usr/bin/restic");
        cmd.arg(subcommand);
        Restic { cmd }
    }

    /// Redirects stdout to /dev/null if `quiet == true`
    fn quiet(mut self, quiet: bool) -> Self {
        if quiet {
            self.cmd.stdout(std::process::Stdio::null());
        }
        self
    }

    /// Runs the restic command
    fn run(mut self) {
        self.cmd
            .spawn()
            .expect("Failed running the restic command")
            .wait()
            .unwrap();
    }
}

pub fn init(app: &App) {
    let repo_name = &app.config.backup.repo_name;
    let repo = app.config.repos.get(repo_name).unwrap();

    let mut restic = Restic::new("init");
    restic
        .cmd
        .args(["--repo", &repo.path])
        .args(["--password-file", &repo.pw_file]);

    restic.run();
}

pub fn backup(app: &App) {
    let repo_name = &app.config.backup.repo_name;
    let repo = app
        .config
        .repos
        .get(repo_name)
        .unwrap_or_else(|| panic!("Failed to find {} in repo map", repo_name));

    let mut restic = Restic::new("backup");
    restic
        .cmd
        .args(["--repo", &repo.path])
        .args(["--password-file", &repo.pw_file])
        .arg("--exclude-caches");

    if app.args.dry_run {
        restic.cmd.arg("--dry-run=true");
    }

    if let Some(excludes) = app.config.backup.exclude.as_ref() {
        for e in excludes {
            restic.cmd.args(["--exclude", e]);
        }
    };

    app.config.backup.include.iter().for_each(|inc| {
        restic.cmd.arg(inc);
    });

    if let Some(cmd) = &app.config.backup.pre_command {
        run_cmd(cmd)
    }

    restic.quiet(app.args.quiet).run();

    if let Some(cmd) = &app.config.backup.post_command {
        run_cmd(cmd)
    }
}

pub fn check(app: &App, repo_name: String) {
    let repo = &app.config.repos.get(&repo_name).unwrap();

    let mut restic = Restic::new("check");
    restic
        .cmd
        .args(["--repo", &repo.path])
        .args(["--password-file", &repo.pw_file]);

    restic.quiet(app.args.quiet).run();
}

pub fn copy(app: &App, src_repo: String, dest_repo: String) {
    let src_repo = &app.config.repos.get(&src_repo).unwrap();
    let dest_repo = &app.config.repos.get(&dest_repo).unwrap();

    let mut restic = Restic::new("copy");
    restic
        .cmd
        .args(["--repo", &src_repo.path])
        .args(["--password-file", &src_repo.pw_file])
        .args(["--repo2", &dest_repo.path])
        .args(["--password-file2", &dest_repo.pw_file]);

    if app.args.dry_run {
        restic.cmd.arg("--dry-run=true");
    }

    if let Some(copy) = &app.config.copy {
        if let Some(cmd) = &copy.pre_command {
            run_cmd(cmd)
        }
    }

    restic.quiet(app.args.quiet).run();

    if let Some(copy) = &app.config.copy {
        if let Some(cmd) = &copy.post_command {
            run_cmd(cmd)
        }
    }
}

pub fn forget(app: &App, repo_name: String) {
    let repo = &app.config.repos.get(&repo_name).unwrap();

    let mut restic = Restic::new("forget");
    restic
        .cmd
        .args(["--repo", &repo.path])
        .args(["--password-file", &repo.pw_file])
        .arg("--prune");

    if app.args.dry_run {
        restic.cmd.arg("--dry-run=true");
    }

    if let Some(t) = &app.config.forget.keep_yearly {
        restic.cmd.args(["--keep-yearly", &t.to_string()]);
    }
    if let Some(t) = &app.config.forget.keep_monthly {
        restic.cmd.args(["--keep-monthly", &t.to_string()]);
    }
    if let Some(t) = &app.config.forget.keep_weekly {
        restic.cmd.args(["--keep-weekly", &t.to_string()]);
    }
    if let Some(t) = &app.config.forget.keep_daily {
        restic.cmd.args(["--keep-daily", &t.to_string()]);
    }

    restic.quiet(app.args.quiet).run();
}

pub fn snapshots(app: &App, repo_name: String) {
    let repo = &app.config.repos.get(&repo_name).unwrap();

    let mut restic = Restic::new("snapshots");
    restic
        .cmd
        .args(["--repo", &repo.path])
        .args(["--password-file", &repo.pw_file]);

    restic.quiet(app.args.quiet).run();
}

pub fn mount(app: &App, repo_name: String, mount_point: String) {
    let repo = &app.config.repos.get(&repo_name).unwrap();

    let mut restic = Restic::new("mount");
    restic
        .cmd
        .arg(mount_point)
        .args(["--repo", &repo.path])
        .args(["--password-file", &repo.pw_file]);

    if app.args.dry_run {
        restic.cmd.arg("--dry-run=true");
    }

    restic.quiet(app.args.quiet).run();
}

pub fn prune(app: &App, repo_name: String) {
    let repo = &app.config.repos.get(&repo_name).unwrap();

    let mut restic = Restic::new("prune");
    restic
        .cmd
        .args(["--repo", &repo.path])
        .args(["--password-file", &repo.pw_file]);

    if app.args.dry_run {
        restic.cmd.arg("--dry-run=true");
    }

    restic.quiet(app.args.quiet).run();
}

pub fn unlock(app: &App, repo_name: String) {
    let repo = &app.config.repos.get(&repo_name).unwrap();

    let mut restic = Restic::new("unlock");
    restic
        .cmd
        .args(["--repo", &repo.path])
        .args(["--password-file", &repo.pw_file]);

    if app.args.dry_run {
        restic.cmd.arg("--dry-run=true");
    }

    restic.quiet(app.args.quiet).run();
}

pub fn stats(app: &App, repo_name: String, snapshot_id: Option<String>) {
    let repo = &app.config.repos.get(&repo_name).unwrap();

    let mut restic = Restic::new("stats");
    restic
        .cmd
        .args(["--repo", &repo.path])
        .args(["--password-file", &repo.pw_file]);

    if let Some(s) = snapshot_id {
        restic.cmd.arg(s);
    }

    restic.quiet(app.args.quiet).run();
}

pub fn run_cmd(cmd_str: &str) {
    let mut cmd = Command::new("/bin/bash");
    cmd.arg("-c").arg(&cmd_str);

    match cmd.spawn() {
        Ok(mut cmd) => {
            cmd.wait().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to run command '{}'. {}", cmd_str, e);
            exit(1);
        }
    }
}
