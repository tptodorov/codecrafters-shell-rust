use std::cell::RefCell;
use std::collections::HashMap;

type ReturnCode = u8;

pub struct Context<'a> {
    pub last_code: ReturnCode,
    pub builtins: &'a Builtins,
}

// This is a function pointer to a function executed by the shell.
// Ok(code) result means the function was executed and returned code.
// Err(code) means the function terminates the shell with code.
pub type BuiltInFn = Box<(dyn Fn(Context, &[&str]) -> Result<ReturnCode, ReturnCode>)>;

pub type Builtins = HashMap<&'static str, RefCell<BuiltInFn>>;

pub fn exit(c: Context, args: &[&str]) -> Result<ReturnCode, ReturnCode> {
    match args {
        [code] => {
            let code = code.parse::<u8>();
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
                println!("{} not found", name);
            }
            SUCCESS
        }
        _ => {
            println!("type function requires arguments");
            Ok(1)
        }
    }
}

const SUCCESS: Result<ReturnCode, ReturnCode> = Ok(0);