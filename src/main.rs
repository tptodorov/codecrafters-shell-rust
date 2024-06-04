use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::ExitCode;
use crate::builtins::{Builtins, Context};

mod builtins;

fn main() -> ExitCode {
    let mut builtins: Builtins = HashMap::new();

    // register all builtins
    builtins.insert("exit", RefCell::new(Box::new(builtins::exit)));
    builtins.insert("echo", RefCell::new(Box::new(builtins::echo)));
    builtins.insert("type", RefCell::new(Box::new(builtins::type_cmd)));

    let mut last_code = 0;
    let mut input = String::new();
    let stdin = io::stdin();

    loop {
        input.clear();

        print!("$ ");
        io::stdout().flush().unwrap();

        stdin.read_line(&mut input).unwrap();

        let input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }

        let cmd = input.split_ascii_whitespace().collect::<Vec<&str>>();

        match &cmd[..] {
            [cmd, args @ .. ] => {
                if let Some(func) = builtins.get(cmd.trim()) {
                    let func = func.borrow();
                    match func(Context{ last_code, builtins: &builtins },args) {
                        Ok(code) => {
                            last_code = code;
                        }
                        Err(code) => return ExitCode::from(code)
                    }
                } else {
                    last_code = 1;
                    println!("{}: command not found", input);
                }
            }
            _ => {
                panic!("invalid command: {}", input);
            }
        }
    }
}
