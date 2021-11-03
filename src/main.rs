mod commit_message;
mod config;

use std::io::Write;
use std::process::Command;

use commit_message::make_message_commit;

fn main() {
    let pattern = config::get_pattern();
    let commit_message = make_message_commit(&pattern);

    match commit_message {
        Ok(message) => {
            let output = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(message)
                .output()
                .expect("Failed to execute process");
            std::io::stdout().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
            std::process::exit(output.status.code().unwrap());
        }
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    }
}
