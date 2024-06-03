#[allow(unused_imports)]
use std::io::{ self, Write };
use std::process::Command;

fn main() {
    let stdin = io::stdin();
    let path_env = std::env::var("PATH").unwrap();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let argv: Vec<&str> = input.trim().split_whitespace().collect();

        if argv.is_empty() {
            continue;
        } else if let Some(path) = path_finder(argv[0], &path_env) {
            Command::new(path)
                .args(&argv[1..])
                .status()
                .expect("failed to execute process");
        } else {
            let builtins = ["exit", "echo", "type"];

            match argv[0] {
                "exit" | "exit 0" => {
                    break;
                }
                "echo" => {
                    println!("{}", argv[1..].join(" "));
                }
                "type" => {
                    if argv.len() != 2 {
                        println!("type: expected 1 argument, got {}", argv.len() - 1);
                        continue;
                    }
                    let cmd = argv[1];
                    if builtins.contains(&cmd) {
                        println!("{} is a shell builtin", cmd);
                    } else {
                        if let Some(path) = path_finder(cmd, &path_env) {
                            println!("{} is {}", cmd, path);
                        } else {
                            println!("{} not found", cmd);
                        }
                    }
                }
                _ => println!("{}: command not found", argv[0]),
            }
        }
    }
}

fn path_finder(command: &str, path_env: &str) -> Option<String> {
    for path in path_env.split(":") {
        let full_path = format!("{}/{}", path, command);
        if std::path::Path::new(&full_path).exists() {
            return Some(full_path);
        }
    }
    None
}
