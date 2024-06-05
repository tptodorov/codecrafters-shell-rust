use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

type ReturnCode = i32;

/// Shell Execution context
pub struct Context<'a> {
    pub(crate) last_code: ReturnCode,
    builtins: &'a Builtins,
    current: PathBuf,
}

impl<'a> Context<'a> {
    pub fn new(current: PathBuf, builtins: &'a Builtins) -> Self {
        Self {
            last_code: 0,
            current,
            builtins,
        }
    }

    pub fn home(&self) -> PathBuf {
        PathBuf::from(env::var("HOME").unwrap_or_default())
    }

    /// find a file in current path
    pub fn find_file(&self, name: &str) -> Option<PathBuf> {
        env::var("PATH")
            .unwrap_or_default()
            .split(":")
            .map(|s| PathBuf::from(s))
            .find(|p| p.join(name).is_file())
            .map(|p| p.join(name))
    }
}

// This is a function pointer to a function executed by the shell.
// Ok(code) result means the function was executed and returned code.
// Err(code) means the function terminates the shell with code.
pub type BuiltinFn = Box<(dyn Fn(&mut Context, &[&str]) -> Result<ReturnCode, ReturnCode>)>;

pub type Builtins = HashMap<&'static str, RefCell<BuiltinFn>>;

const SUCCESS: Result<ReturnCode, ReturnCode> = Ok(0);

pub fn exit(c: &mut Context, args: &[&str]) -> Result<ReturnCode, ReturnCode> {
    match args {
        [code] => {
            let code = code.parse::<ReturnCode>();
            let code = code.unwrap_or_default();
            Err(code)
        }
        _ => Err(c.last_code),
    }
}

pub fn echo(_c: &mut Context, args: &[&str]) -> Result<ReturnCode, ReturnCode> {
    println!("{}", args.join(" ").as_str());
    SUCCESS
}

pub fn pwd(c: &mut Context, _: &[&str]) -> Result<ReturnCode, ReturnCode> {
    println!("{}", c.current.to_string_lossy());
    SUCCESS
}

pub fn cd(c: &mut Context, args: &[&str]) -> Result<ReturnCode, ReturnCode> {
    match args {
        [] | ["~"] => {
            c.current = c.home();
            SUCCESS
        }
        [destination] => {
            match c.current.join(Path::new(destination)).canonicalize() {
                Ok(target) => {
                    if target.is_dir() {
                        c.current = target;
                        SUCCESS
                    } else {
                        println!("{}: No such file or directory", target.to_string_lossy());
                        Ok(1)
                    }
                }
                Err(_) => {
                    println!("{}: No such file or directory", destination);
                    Ok(1)
                }
            }
        }
        _ => Ok(1),
    }
}

pub fn type_cmd(c: &mut Context, args: &[&str]) -> Result<ReturnCode, ReturnCode> {
    match args {
        [name] => {
            if c.builtins.contains_key(name) {
                println!("{} is a shell builtin", name);
            } else {
                if let Some(found) = c.find_file(name) {
                    println!("{} is {}", name, found.to_string_lossy());
                } else {
                    println!("{} not found", name);
                }
            }
            SUCCESS
        }
        _ => {
            println!("type function requires arguments");
            Ok(1)
        }
    }
}

