use colored::Colorize;
use std::{env, path, process};

pub struct Args {
    pub asm_path: String,
}

impl Args {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        let asm_path = match args.get(1) {
            Some(p) => {
                if !path::Path::new(p).exists() {
                    quit(&format!("File not found: '{}'", p), 1);
                };
                p
            }
            // TODO: help text
            None => quit("IAS code not provided", 1),
        };

        Self {
            asm_path: asm_path.to_string(),
        }
    }
}

/// Quit the program with an error message.
/// TODO: Show line for warnings and errors in the assembly code.
pub fn quit(msg: &str, code: i32) -> ! {
    eprintln!("{}: {}", "Error".red().bold(), msg);
    process::exit(code);
}

/// Print a warning message during indexing phase.
pub fn warn(msg: &str) {
    eprintln!("{}: {}", "Warning".yellow(), msg);
}
