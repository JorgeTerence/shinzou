mod cli;
mod ias;

use cli::Args;
use ias::{Command, Instruction};
use std::fs;
use std::process;

fn main() {
    let args = Args::new();

    let mut program: Vec<Instruction> = vec![];

    match fs::read_to_string(args.asm_path) {
        Ok(code) => {
            for line in code.lines() {
                // Remove comments
                let (line, _) = line.split_once("--").unwrap_or((line, ""));

                program.push(Instruction::new(line));
                // TODO: binary code
            }
        }
        Err(_) => quit("Erro ao ler arquivo do programa.", 1),
    }

    println!(
        "{:?}",
        program
            .iter()
            .filter(|i| matches!(i.call, Command::Directive(_)))
            .map(|i| format!("{} {}", i.call, i.arg))
            .collect::<Vec<String>>()
    );
}

fn quit(msg: &str, code: i32) -> ! {
    eprintln!("{}", msg);
    process::exit(code);
}
