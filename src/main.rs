#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
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
            echo(input);
        }
        _ => {
            println!("{input}: command not found");
        }
    }
}

fn echo(input: &str) {
    if input.len() <= 4 {
        println!("{input}: command not found");
        return;
    }
    let input = &input[5..];
    println!("{input}");
}
