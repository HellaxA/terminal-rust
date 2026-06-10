use std::process::Command;
use std::str::SplitWhitespace;

use crate::{find_executable_in_path, format_not_found};

pub fn run_execute(input: &str, first_arg: &str, arg_iterator: SplitWhitespace) -> String {
    let option_result = find_executable_in_path(first_arg);

    if option_result.is_none() {
        return format_not_found(&input);
    }

    let output = Command::new(first_arg).args(arg_iterator).output().unwrap();

    return String::from_utf8(output.stdout)
        .unwrap()
        .trim_end()
        .to_string();
}

pub mod test {}
