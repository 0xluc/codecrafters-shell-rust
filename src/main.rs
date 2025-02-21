use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut paths = String::from("");
    match env::var("PATH") {
        Ok(v) => paths = v,
        Err(e) => println!("{}", e),
    }
    let path_vec: Vec<&str> = paths.split(":").collect();

    'outer: loop {
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
                let no_type_string = &command_tokens[1..command_tokens.len()].join(" ");
                for path in &path_vec {
                    match fs::metadata(format!("{path}/{no_type_string}")) {
                        Ok(_) => {
                            println!("{} is {}", no_type_string, path);
                            continue 'outer;
                        }
                        Err(_) => {}
                    }
                }
                println!("{}: not found", no_type_string);
            }
            _ => {
                println!("{}: command not found", command);
            }
        }
    }
}
