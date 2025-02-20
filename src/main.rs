#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = input[0..input.len() - 1].to_string();
        let command_tokens: Vec<&str> = command.split_whitespace().collect();
        if command_tokens[0] == "exit" {
            std::process::exit(command_tokens[1].parse::<i32>().unwrap());
        }

        if command_tokens[0] == "echo" {
            if command_tokens.len() < 2 {
                println!("");
                continue;
            }
            let no_echo = &command_tokens[1..command_tokens.len()];
            println!("{}", no_echo.join(" "));
            continue;
        }

        println!("{}: command not found", command);
    }
}
