use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut last_status = 0;
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let cmd = input.split_whitespace().collect::<Vec<&str>>();
        match &cmd[..] {
            ["exit"] => break,
            ["exit", code] => {
                let code = code.parse::<u8>();
                last_status = code.unwrap_or_default();
                break;
            }
            _ => {
                last_status = 1;
                println!("{}: command not found", input);
            }
        }
    }
    return ExitCode::from(last_status);
}
