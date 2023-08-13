use std::process::{Command, Output};

pub struct Git {}

impl Git {
    pub fn add(file_name: &str) -> Output {
        Command::new("git")
            .arg("add")
            .arg(file_name)
            .output()
            .expect("failed to execute git add command")
    }

    pub fn commit(message: &str) -> Output {
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .expect("failed to execute git commit command")
    }
}
