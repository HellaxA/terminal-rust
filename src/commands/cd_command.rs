use std::env;
use std::path::Path;

const ARG_START: usize = 3;

pub fn run_cd(command: &str, input: &str) -> String {
    if input.len() < ARG_START {
        return format!("{input}: No such file or directory");
    }
    let argument = &input[ARG_START..];

    let path = Path::new(argument);
    let is_changed = env::set_current_dir(path).is_ok();

    if !is_changed {
        return format!("{command}: {argument}: No such file or directory");
    }

    return String::new();
}
