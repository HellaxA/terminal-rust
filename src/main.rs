use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use std::sync::OnceLock;

const ECHO_MIN_LEN: usize = 4;
const ECHO_ARGS_START: usize = 5;
const TYPE_MIN_LEN: usize = 4;
const TYPE_ARG_START: usize = 5;
static COMMANDS: OnceLock<HashSet<&str>> = OnceLock::new();

fn main() {
    init();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        process_input(input);
    }
}

fn init() {
    COMMANDS
        .set(HashSet::from(["exit", "echo", "type"]))
        .unwrap();
}

fn process_input(input: &str) {
    let mut split_input = input.split_whitespace();
    let command = split_input.next().unwrap_or("invalid");

    match command {
        "exit" => {
            process::exit(0);
        }
        "echo" => {
            run_echo(input);
        }
        "type" => {
            run_type(input);
        }
        _ => {
            println!("{input}: command not found");
        }
    }
}

fn run_type(input: &str) {
    if input.len() <= TYPE_MIN_LEN {
        println!("{input}: command not found");
        return;
    }

    let argument = &input[TYPE_ARG_START..];

    if COMMANDS.get().unwrap().contains(argument) {
        println!("{argument} is a shell builtin");
    } else {
        println!("{argument}: not found");
    }
}

fn run_echo(input: &str) {
    if input.len() <= ECHO_MIN_LEN {
        println!("{input}: command not found");
        return;
    }
    let input = &input[ECHO_ARGS_START..];
    println!("{input}");
}
