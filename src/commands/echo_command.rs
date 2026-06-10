use crate::format_not_found;

const MIN_LEN: usize = 4;
const ARGS_START: usize = 5;

pub fn run_echo(input: &str) -> String {
    let input = input.trim();
    if input.len() <= MIN_LEN {
        return format_not_found(input);
    }
    input[ARGS_START..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_no_args() {
        let result = run_echo("echo");

        assert_eq!("echo: command not found", result);
    }
    #[test]
    fn echo_one_arg() {
        let result = run_echo("echo hello");

        assert_eq!("hello", result);
    }
    #[test]
    fn echo_multiple_args() {
        let result = run_echo("echo hello foo bar");

        assert_eq!("hello foo bar", result);
    }

    #[test]
    fn echo_spaces_around() {
        let result = run_echo(" echo foo bar ");

        assert_eq!("foo bar", result);
    }

    #[test]
    fn echo_empty() {
        let result = run_echo("");

        assert_eq!(": command not found", result);
    }
}
