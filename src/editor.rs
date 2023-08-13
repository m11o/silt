use std::process::{Command, ExitStatus};
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Editor<'a> {
    editor_type: &'a str,
}

impl<'a> Editor<'a> {
    pub fn new(editor_type: &'a str) -> Self {
        Self { editor_type }
    }

    pub fn open(&self, file_path: &str) -> ExitStatus {
        self.create_file_if_not_exists(file_path);

        match self.editor_type {
            "vim" => {
                Command::new("vim")
                    .arg(file_path)
                    .status()
                    .expect("failed to open vim")
            }
            "nvim" => {
                Command::new("nvim")
                    .arg(file_path)
                    .status()
                    .expect("failed to open nvim")
            }
            _ => {
                Command::new("nvim")
                    .arg(file_path)
                    .status()
                    .expect("failed to open vim")
            }
        }
    }

    fn create_file_if_not_exists(&self, file_path: &str) {
        if !Path::new(file_path).exists() {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(file_path)
                .expect("Failed to create new file.");
            writeln!(file, "").expect("Failed to write to new file.");
        }
    }
}
