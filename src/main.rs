use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let methods = ["exit", "echo", "type", "pwd", "cd"];
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
                if methods.iter().any(|&method| method == no_type_string) {
                    println!("{} is a shell builtin", no_type_string);
                    continue;
                }
                for path in &path_vec {
                    match fs::metadata(format!("{path}/{no_type_string}")) {
                        Ok(_) => {
                            println!("{} is {}/{}", no_type_string, path, no_type_string);
                            continue 'outer;
                        }
                        Err(_) => {}
                    }
                }
                println!("{}: not found", no_type_string);
            }
            "pwd" => match std::env::current_dir() {
                Ok(v) => {
                    println!("{}", v.to_string_lossy());
                }
                Err(_) => {}
            },
            "cd" => {
                if &command_tokens[1] == &"~" {
                    let mut home_path = String::from("");
                    match env::var("HOME") {
                        Ok(v) => {
                            home_path = v;
                            match std::env::set_current_dir(&home_path) {
                                Ok(_) => {}
                                Err(_) => {
                                    println!(
                                        "cd: {}: No such file or directory",
                                        &command_tokens[1]
                                    )
                                }
                            }
                        }
                        Err(e) => println!("{}", e),
                    }
                } else {
                    match std::env::set_current_dir(&command_tokens[1]) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("cd: {}: No such file or directory", &command_tokens[1])
                        }
                    }
                }
            }
            _ => {
                let command_program = &command_tokens[0];
                if command_tokens.len() > 1 {
                    for path in &path_vec {
                        match fs::metadata(format!("{path}/{command_program}")) {
                            Ok(_) => {
                                if let Some((program, rest)) = command_tokens.split_first() {
                                    let output = Command::new(program)
                                        .args(rest)
                                        .output()
                                        .expect("Failed to execute!");
                                    print!("{}", String::from_utf8_lossy(output.stdout.as_slice()));
                                }
                                continue 'outer;
                            }
                            Err(_) => {}
                        }
                    }
                }

                println!("{}: command not found", command);
            }
        }
    }
}
