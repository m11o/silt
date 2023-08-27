mod dairy_manager;
mod editor;
mod git;

use chrono::Local;
use std::env;
use std::path::Path;

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
        let current_dir = env::current_dir().expect("Failed to get current directory.");
        let current_path_instance = Path::new(current_dir.to_str().unwrap());
        let root_dir = env::var("SILT_PROJECT_ROOT_PATH").expect("Failed to load project root path from env");
        let root_path_instance = Path::new(root_dir.as_str());

        env::set_current_dir(&root_path_instance).expect("Failed to set current directory.");
        Git::add(ref_file_path);
        Git::commit(commit_message(ref_file_path).as_str());
        env::set_current_dir(&current_path_instance).expect("Failed to set current directory.");
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
