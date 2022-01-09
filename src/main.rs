mod commit_message;
mod config;

use anyhow::Result;
use clap::Parser;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use commit_message::make_message_commit;

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
}

fn main() -> Result<()> {
    // Dumb hack to set the current/working directory (pwd) because appimage or cargo-appimage sucks
    // https://github.com/AppImage/AppImageKit/issues/172
    if let Ok(current_dir) = std::env::var("OWD") {
        std::env::set_current_dir(current_dir)?;
    }

    let opt = Opt::parse();
    let pattern = config::get_pattern(opt.config)?;
    let commit_message = make_message_commit(pattern)?;

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("Failed to execute process");
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
    std::process::exit(output.status.code().unwrap());
}
