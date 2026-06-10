use is_executable::is_executable;
use std::{env, path::PathBuf};

pub mod commands;

pub fn format_not_found(input: &str) -> String {
    return format!("{input}: command not found");
}

pub fn find_executable_in_path(executable_name: &str) -> Option<String> {
    let path = env::var("PATH").unwrap_or(String::new());

    let dirs_to_check = env::split_paths(&path);
    for dir in dirs_to_check {
        let file_path_buf = PathBuf::from(dir).join(executable_name);
        let file_path_str = file_path_buf.to_str().unwrap();
        let file_path = file_path_buf.as_path();

        if is_executable(file_path) {
            return Some(String::from(file_path_str));
        }
    }

    return None;
}
