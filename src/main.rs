mod cli;
mod ias;

use cli::{Args, quit};
use ias::{Command, Instruction};
use std::fs;

fn main() {
    let args = Args::new();

    let mut program: Vec<Instruction> = vec![];

    // Assembling
    // Transform strings into symbols -> directives, labels and operators
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

    // Indexing
    // Set memory layout, clean-up symbols, leave only operators
    //
    // * Go line by line counting the memory position
    // * Use .org to set index of memory
    // * Associate labels to addresses
    // * Loop through everything replacing labels
    // * Check for missing labels

    // Compiling
    // Translate symbols into binary code

    // Executing
    // Read memory line-by-line and interpret commands

    // Post-processing
    // Show logs

    println!(
        "{:?}",
        program
            .iter()
            .filter(|i| matches!(i.call, Command::Label(_)))
            .map(|i| i.call.to_string())
            .collect::<Vec<_>>()
    );
}
