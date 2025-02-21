#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let methods = ["exit", "echo", "type"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = input[0..input.len() - 1].to_string();
        let command_tokens: Vec<&str> = command.split_whitespace().collect();
        match command_tokens[0] {
            "exit" => {
                std::process::exit(command_tokens[1].parse::<i32>().unwrap());
            }
            "echo" => {
                if command_tokens.len() < 2 {
                    println!("");
                } else {
                    println!("{}", &command_tokens[1..command_tokens.len()].join(" "));
                }
            }
            "type" => {
                let no_type_string = &command_tokens[1..command_tokens.len()];
                if methods
                    .iter()
                    .any(|&method| method == &no_type_string.join(" "))
                {
                    println!("{} is a shell builtin", no_type_string.join(" "));
                } else {
                    println!("{}: not found", no_type_string.join(" "));
                }
            }
            _ => {
                println!("{}: command not found", command);
            }
        }
    }
}
