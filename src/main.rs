mod dairy_manager;
mod editor;
mod git;

use chrono::Local;

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
        Git::add(ref_file_path);
        Git::commit(commit_message(ref_file_path).as_str());
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
