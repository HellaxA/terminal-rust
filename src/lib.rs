use is_executable::is_executable;
use std::collections::HashSet;
use std::env;
use std::path::{MAIN_SEPARATOR, PathBuf};
use std::sync::OnceLock;

const ECHO_MIN_LEN: usize = 4;
const ECHO_ARGS_START: usize = 5;
const TYPE_MIN_LEN: usize = 4;
const TYPE_ARG_START: usize = 5;

pub static COMMANDS: OnceLock<HashSet<&str>> = OnceLock::new();

pub fn init() {
    COMMANDS
        .set(HashSet::from(["exit", "echo", "type"]))
        .unwrap();
}

pub fn run_type(input: &str) {
    if input.len() <= TYPE_MIN_LEN {
        print_not_found(input);
        return;
    }

    let argument = &input[TYPE_ARG_START..];

    if COMMANDS.get().unwrap().contains(argument) {
        println!("{argument} is a shell builtin");
    // Check if it is executable
    } else {
        let path = env::var("PATH").unwrap_or(String::new());

        let dirs_to_check = path.split(MAIN_SEPARATOR);
        for dir in dirs_to_check {
            let file_path_buf = PathBuf::from(dir).join(argument);
            let file_path_str = file_path_buf.to_str().unwrap();
            let file_path = file_path_buf.as_path();

            if is_executable(file_path) {
                println!("{argument} is {file_path_str}");
                return;
            }
        }

        println!("{argument}: not found");
    }
}

pub fn run_echo(input: &str) {
    if input.len() <= ECHO_MIN_LEN {
        print_not_found(input);
        return;
    }
    let input = &input[ECHO_ARGS_START..];
    println!("{input}");
}

pub fn print_not_found(input: &str) {
    println!("{input}: command not found");
}
