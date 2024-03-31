use colored::Colorize;
use std::{env, path, process};

pub struct Args {
    pub asm_path: String,
    pub bin_path: String,
}

enum Init {
    RunIAS,
    RunBin,
    Compile,
    Help,
}

impl Args {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            help();
            process::exit(0);
        }

        let mut asm_path = None;

        match args[1].as_str() {
            "run" => {
                asm_path = match args.get(2) {
                    Some(p) => {
                        if !path::Path::new(p).exists() {
                            quit(&format!("File not found: '{}'", p), 1);
                        };
                        Some(p.as_str())
                    }
                    None => quit("No file selected", 1),
                }
            }
            "compile" => (),
            _ => {
                help();
                process::exit(0);
            }
        }

        Self {
            asm_path: asm_path,
            bin_path: format!("{}.bin", asm_path),
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
/// TODO: Implement warning suppression.
pub fn warn(msg: &str) {
    eprintln!("{}: {}", "Warning".yellow(), msg);
}

fn help() {
    print!(
        "Usage: [COMMAND] <IAS_FILE> [OPTIONS]
Commands:
    run       Run either .ias or .bin file
    compile   Compile IAS code to a binary file
Options:
    -o, --output <FILE>    Output file name for compile command
    --suppress-warnings    Suppress warnings during indexing phase"
    )
}
