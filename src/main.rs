mod commit_message;
mod config;

use anyhow::Result;
use clap::Parser;
use std::io::Write;
use std::path::PathBuf;
use std::process::{exit, Command};

use commit_message::make_message_commit;

const DEFAULT_CONFIG_FILE: &str = include_str!("../commit-default.json");

#[derive(Parser)]
#[clap(about, author, version)]
struct Opt {
    #[clap(
        short,
        long,
        help = "Custom configuration file path",
        parse(from_os_str)
    )]
    config: Option<PathBuf>,
    #[clap(long, help = "Init custom configuration file")]
    init: bool,
}

fn main() -> Result<()> {
    // Dumb hack to set the current/working directory (pwd) because appimage or cargo-appimage sucks
    // https://github.com/AppImage/AppImageKit/issues/172
    if let Ok(current_dir) = std::env::var("OWD") {
        std::env::set_current_dir(current_dir)?;
    }

    let opt = Opt::parse();
    if opt.init {
        let mut file = std::fs::File::create("commit.json")?;
        file.write_all(DEFAULT_CONFIG_FILE.as_bytes())?;
        Ok(())
    } else {
        let pattern = config::get_pattern(opt.config)?;
        let commit_message = make_message_commit(pattern)?;

        let output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit_message)
            .output();
        match output {
            Ok(output) => {
                std::io::stdout().write_all(&output.stdout).unwrap();
                std::io::stderr().write_all(&output.stderr).unwrap();
                exit(output.status.code().unwrap());
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to run git. Make sure git is installed\nAdditional info: {}",
                    e
                ));
            }
        };
    }
}
