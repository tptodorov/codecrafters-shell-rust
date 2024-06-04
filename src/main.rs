use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use crate::builtins::{Builtins, Context};

mod builtins;


fn main() -> ExitCode {
    let mut builtins: Builtins = HashMap::new();

    // register all builtins
    builtins.insert("exit", RefCell::new(Box::new(builtins::exit)));
    builtins.insert("echo", RefCell::new(Box::new(builtins::echo)));
    builtins.insert("type", RefCell::new(Box::new(builtins::type_cmd)));
    builtins.insert("pwd", RefCell::new(Box::new(builtins::pwd)));
    builtins.insert("cd", RefCell::new(Box::new(builtins::cd)));

    let mut input = String::new();
    let stdin = io::stdin();
    let path = &path();
    let current = env::current_dir().unwrap();
    let mut context = Context { last_code: 0, builtins: &builtins, path, current };

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
                    match func(&mut context, args) {
                        Ok(code) => {
                            context.last_code = code;
                        }
                        Err(code) => return ExitCode::from(code as u8)
                    }
                } else if let Some(exec) = context.find_file(cmd) {
                    match std::process::Command::new(exec).args(args).status() {
                        Ok(status) => {
                            context.last_code = status.code().unwrap_or_default();
                        }
                        Err(_) => {}
                    }
                } else {
                    context.last_code = 1;
                    println!("{}: command not found", input);
                }
            }
            _ => {
                panic!("invalid command: {}", input);
            }
        }
    }
}

fn path() -> Vec<PathBuf> {
    env::var("PATH").unwrap_or_default().split(":").map(|s| PathBuf::from(s)).collect()
}
