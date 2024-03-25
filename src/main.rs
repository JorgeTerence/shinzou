mod cli;
mod ias;

use cli::Args;
use ias::Instruction;
use std::fs;
use std::process;

fn main() {
    let args = Args::new();

    let program: Vec<Instruction> = vec![];

    match fs::read_to_string(args.asm_path) {
        Ok(program) => {
            for line in program.lines() {
                // Remove comments
                let (line, _) = line.split_once("--").unwrap_or((line, ""));

                let instruc = Instruction::new(line);
                // TODO: binary code
            }
        }
        Err(_) => quit("Erro ao ler arquivo do programa.", 1),
    }
}

fn quit(msg: &str, code: i32) -> ! {
    eprintln!("{}", msg);
    process::exit(code);
}
