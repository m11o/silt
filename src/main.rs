mod dairy_manager;
mod editor;
mod git;

use chrono::Local;
use std::env;
use std::path::{PathBuf};

use dairy_manager::DairyManager;
use editor::Editor;
use crate::git::Git;

fn main() {
    let now = Local::now().date_naive();
    let dairy_manager = DairyManager::new(&now);
    let file_path = dairy_manager.build_file_path();
    let ref_file_path = file_path.as_str();

    let editor = Editor::new("nvim");
    let status = editor.open(ref_file_path);
    if status.success() {
        fn git_add_and_commit_dairy_file(file_path: &str) {
            Git::add(file_path);
            Git::commit(commit_message(file_path).as_str());
        }
        change_current_to_root_dir(git_add_and_commit_dairy_file, ref_file_path);
    } else {
        println!("Failed to open file.");
    }
}

fn commit_message(file_path: &str) -> String {
    let now_datetime = Local::now();
    let mut message = file_path.to_string();
    message.push_str(format!(": {}", now_datetime.format("%Y-%m-%d %H:%M:%S")).as_str());
    message
}

fn change_current_to_root_dir<F>(function: F, file_name: &str) where F: Fn(&str) -> () {
    env::set_current_dir(root_dir().as_path()).expect("Failed to set current directory.");
    function(file_name);
    env::set_current_dir(current_dir().as_path()).expect("Failed to set current directory.");
}

fn current_dir() -> PathBuf {
    match env::current_dir() {
        Ok(path) => path,
        Err(_) => panic!("Failed to get current directory."),
    }
}

fn root_dir() -> PathBuf {
    match env::var("SILT_PROJECT_ROOT_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => panic!("Failed to load project root path from env"),
    }
}
