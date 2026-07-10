use std::env;
use std::path::PathBuf;

const ARG_START: usize = 3;

pub fn run_cd(command: &str, input: &str) -> String {
    if input.len() < ARG_START {
        return format!("{input}: No such file or directory");
    }
    let argument = &input[ARG_START..];

    let path: PathBuf = if argument == "~" {
        env::home_dir().unwrap()
    } else {
        PathBuf::from(argument)
    };

    let is_changed = env::set_current_dir(&path).is_ok();

    if !is_changed {
        return format!("{command}: {argument}: No such file or directory");
    }

    return String::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cd_short_arg() {
        let result = run_cd("cd", "cd");
        assert_eq!("cd: No such file or directory", result);
    }

    #[test]
    fn test_cd_dir_not_set() {
        let result = run_cd("cd", "cd /asdf");
        assert_eq!("cd: /asdf: No such file or directory", result);
    }
}
