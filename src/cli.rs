use clap::Parser;
use colored::Colorize;
use std::{path::PathBuf, process};

// #[derive(Subcommand)]
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Args {
    Run {
        path: PathBuf,
    },
    Compile {
        asm_path: PathBuf,
        bin_path: Option<PathBuf>,
    },
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
