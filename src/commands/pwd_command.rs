use std::env::current_dir;
pub fn run_pwd() -> String {
    return current_dir().unwrap().to_str().unwrap().to_string();
}
