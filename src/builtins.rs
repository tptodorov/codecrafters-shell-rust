use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

type ReturnCode = i32;

pub struct Context<'a> {
    pub last_code: ReturnCode,
    pub builtins: &'a Builtins,
    pub path: &'a [PathBuf]
}

impl Context<'_> {
    pub fn path(&self, name: &str) -> Option<PathBuf> {
        self.path
            .iter()
            .find(|p| p.join(name).is_file())
            .map(|p| p.join(name))
    }
}

// This is a function pointer to a function executed by the shell.
// Ok(code) result means the function was executed and returned code.
// Err(code) means the function terminates the shell with code.
pub type BuiltInFn = Box<(dyn Fn(Context, &[&str]) -> Result<ReturnCode, ReturnCode>)>;

pub type Builtins = HashMap<&'static str, RefCell<BuiltInFn>>;

const SUCCESS: Result<ReturnCode, ReturnCode> = Ok(0);

pub fn exit(c: Context, args: &[&str]) -> Result<ReturnCode, ReturnCode> {
    match args {
        [code] => {
            let code = code.parse::<ReturnCode>();
            let code = code.unwrap_or_default();
            Err(code)
        }
        _ => Err(c.last_code),
    }
}

pub fn echo(_c: Context, args: &[&str]) -> Result<ReturnCode, ReturnCode> {
    println!("{}", args.join(" ").as_str());
    SUCCESS
}

pub fn type_cmd(c: Context, args: &[&str]) -> Result<ReturnCode, ReturnCode> {
    match args {
        [name] => {
            if c.builtins.contains_key(name) {
                println!("{} is a shell builtin", name);
            } else {
                if let Some(found) = c.path(name) {
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

