mod commit_message;
mod config;

use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use structopt::{clap::AppSettings::ColoredHelp, StructOpt};

use commit_message::make_message_commit;

#[derive(StructOpt)]
#[structopt(
    name = "commit",
    about = "A tool to make patterned (conventional) commit messages",
    author = "Pedro H. M. <pedromendescraft@gmail.com>",
    setting = ColoredHelp,
)]
struct Opt {
    #[structopt(short, long, help = "Custom configuration file path")]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
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
