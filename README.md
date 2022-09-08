# Restic-rs

This is a simple wrapper around Restic to allow configuration via a yaml file. An example configuration file is included.

## Usage

```
USAGE:
    restic-rs [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config-file <FILE>    Alternate configuration file to use [default: /etc/restic-
                                rs/repos.yaml]
    -h, --help                  Print help information
    -n, --dry-run               Do not upload or write any data, just show what would be done
    -q, --quiet                 Suppress standard output
    -V, --version               Print version information

SUBCOMMANDS:
    backup       Run a backup job now
    check        Check the condition of all configured repos
    copy-all     Copy all configured repo pairs
    help         Print this message or the help of the given subcommand(s)
    init         Initialize a new repo
    mount        Mount the repository at the specified location
    prune        Prune repositories
    snapshots    Displays all snapshots available
    stats        Displays statistics about the configured repos
    unlock       Unlocks the specified repo
```
