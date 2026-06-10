use std::collections::HashSet;

use crate::{find_executable_in_path, format_not_found};

const MIN_LEN: usize = 4;
const ARG_START: usize = 5;

pub fn run_type(input: &str, commands: &HashSet<&str>) -> String {
    let input = input.trim();

    if input.len() <= MIN_LEN {
        return format_not_found(input);
    }

    let argument = &input[ARG_START..];

    if commands.contains(argument) {
        return format!("{argument} is a shell builtin");
    // Check if it is executable
    } else {
        let option_result = find_executable_in_path(argument);

        if option_result.is_none() {
            return format!("{argument}: not found");
        }

        return format!("{argument} is {}", option_result.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_emtpy() {
        let commands = HashSet::new();

        let result = run_type("", &commands);
        assert_eq!(": command not found", result);
    }

    #[test]
    fn type_small_length() {
        let commands = HashSet::new();

        let result = run_type("typ", &commands);
        assert_eq!("typ: command not found", result);
    }

    #[test]
    fn type_shell_builtin() {
        let mut commands = HashSet::new();
        commands.insert("type");

        let result = run_type("type type", &commands);

        assert_eq!("type is a shell builtin", result);
    }

    #[test]
    fn type_not_shell_builtin() {
        let commands = HashSet::new();

        let result = run_type("type foo", &commands);

        assert_eq!("foo: not found", result);
    }

    #[test]
    fn type_executable_in_path() {
        let commands = HashSet::new();
        let result = run_type("type cat", &commands);

        assert_eq!("cat is /usr/bin/cat", result);
    }
}
