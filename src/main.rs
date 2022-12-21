#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::str_to_string
)]
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]

mod commit;
mod commit_message;
mod config;

use anyhow::Result;
use clap::Parser;
use std::io::Write;
use std::path::PathBuf;

use commit::{check_staged_files, commit, read_cached_commit, write_cached_commit};
use commit_message::make_message_commit;

const DEFAULT_CONFIG_FILE: &str = include_str!("../commit-default.json");

#[derive(Parser, Debug)]
#[command(about, author, version)]
struct Args {
    /// Custom configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,
    /// Init custom configuration file
    #[arg(long)]
    init: bool,
    /// Use as hook
    #[arg(long)]
    hook: bool,
    /// Retry commit with the same message as the last one
    #[arg(short, long)]
    retry: bool,
}

fn main() -> Result<()> {
    // Dumb hack to set the current/working directory (pwd) because appimage or cargo-appimage sucks
    // https://github.com/AppImage/AppImageKit/issues/172
    if let Ok(current_dir) = std::env::var("OWD") {
        std::env::set_current_dir(current_dir)?;
    }

    check_staged_files()?;

    let args = Args::parse();

    if args.init {
        let mut file = std::fs::File::create("commit.json")?;
        file.write_all(DEFAULT_CONFIG_FILE.as_bytes())?;
        return Ok(());
    }

    if args.retry {
        let commit_message = read_cached_commit()?;
        commit(&commit_message)?;
        return Ok(());
    }

    let pattern = config::get_pattern(args.config)?;
    let commit_message = make_message_commit(pattern)?;
    write_cached_commit(&commit_message)?;

    if args.hook {
        return Ok(());
    }

    commit(&commit_message)?;
    Ok(())
}
