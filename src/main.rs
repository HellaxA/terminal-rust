use codecrafters_shell;
use std::io::{self, Write};
use std::process;

fn main() {
    codecrafters_shell::init();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        process_input(input);
    }
}

fn process_input(input: &str) {
    let mut split_input = input.split_whitespace();
    let command = split_input.next().unwrap_or("invalid");

    match command {
        "exit" => {
            process::exit(0);
        }
        "echo" => {
            codecrafters_shell::run_echo(input);
        }
        "type" => {
            codecrafters_shell::run_type(input);
        }
        _ => {
            codecrafters_shell::print_not_found(input);
        }
    }
}
