use codecrafters_shell::commands::echo_command::run_echo;
use codecrafters_shell::commands::execute_command::run_execute;
use codecrafters_shell::commands::pwd_command::run_pwd;
use codecrafters_shell::commands::type_command::run_type;
use codecrafters_shell::{self};
use std::collections::HashSet;
use std::io::{self, Write};
use std::process;

fn main() {
    let all_commands = HashSet::from(["exit", "echo", "type", "pwd"]);
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        process_input(input, &all_commands);
    }
}

fn process_input(input: &str, all_commands: &HashSet<&str>) {
    let mut split_input = input.split_whitespace();
    let command = split_input.next().unwrap_or("invalid");

    match command {
        "exit" => {
            process::exit(0);
        }
        "echo" => {
            println!("{}", run_echo(input));
        }
        "type" => {
            println!("{}", run_type(input, &all_commands));
        }
        "pwd" => {
            println!("{}", run_pwd());
        }
        _ => {
            println!("{}", run_execute(input, command, split_input));
        }
    }
}
