use chrono::{Datelike, NaiveDate};
use std::fs;
use std::path::PathBuf;
use project_root::get_project_root;

pub struct DairyManager<'a> {
    date: &'a NaiveDate,
}

impl<'a> DairyManager<'a> {
    pub fn new(date: &'a NaiveDate) -> Self {
        let file_dir = Self::_build_file_dir(date);
        fs::create_dir_all(file_dir).expect("Failed to create directory.");

        Self { date }
    }

    pub fn build_file_path(&self) -> String {
        Self::_build_file_path(self.date)
    }

    fn _build_file_path(date: &'a NaiveDate) -> String {
        format!("{}/notes/dairy/{}/{:0>2}.md", Self::project_root_path(), date.year(), date.month())
    }

    fn _build_file_dir(date: &'a NaiveDate) -> String {
        format!("{}/notes/dairy/{}", Self::project_root_path(), date.year())
    }

    fn project_root_path() -> String {
        get_project_root().expect("Failed to get project root.").to_str().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_build_file_path() {
        let date = match NaiveDate::from_ymd_opt(2023, 1, 1) {
            Some(date) => date,
            None => panic!("Failed to create NaiveDate."),
        };
        let dairy_manager = DairyManager::new(&date);
        let expected = "notes/dairy/2023/01.md";

        assert_eq!(dairy_manager.build_file_path(), expected);
    }

    #[test]
    fn it_build_file_dir() {
        let date = match NaiveDate::from_ymd_opt(2023, 1, 1) {
            Some(date) => date,
            None => panic!("Failed to create NaiveDate."),
        };
        let expected = "notes/dairy/2023";

        assert_eq!(DairyManager::_build_file_dir(&date), expected);
    }
}
