use anyhow::Result;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() -> Result<u8> {
    let mut last_status = 0;
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "exit" => break,
            _ => {
                last_status = 1;
                println!("{}: command not found", input);
            }
        }
    }
    return Ok(last_status);
}
