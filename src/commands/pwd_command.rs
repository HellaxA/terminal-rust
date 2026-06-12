use std::env::current_dir;
pub fn run_pwd() -> String {
    return current_dir().unwrap().to_str().unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use crate::commands::cd_command::run_cd;

    use super::*;

    #[test]
    fn test_pwd() {
        run_cd("cd", "cd /home");
        let result = run_pwd();

        assert_eq!("/home", result);
    }
}
