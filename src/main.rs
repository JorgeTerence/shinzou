mod cli;
mod ias;
mod index;

use cli::{quit, Args};
use ias::{Command, Directive, Instruction};
use std::fs;

use crate::index::{collect_definitions, collect_labels, fix_labels};

// TODO: show line for warnings and errors
// TODO: understand other directives
fn main() {
    let args = Args::new();

    let mut program: Vec<Instruction>;

    // Assembling
    // Transform strings into symbols -> directives, labels and operators
    match fs::read_to_string(args.asm_path) {
        Ok(code) => program = assemble(code),
        Err(e) => quit(&format!("Error reading the program: {}", e), 1),
    }

    // Indexing
    // Set memory layout, clean-up symbols, leave only operators
    // let mut memory: [Instruction; 2048];
    let _definititions = collect_definitions(program.clone());
    program.retain(|i| !matches!(i.call, Command::Directive(Directive::Set)));

    let labels = collect_labels(program.clone());
    program.retain(|i| !matches!(i.call, Command::Label(_)));

    // swap both definitions' and labels' values
    program = program
        .into_iter()
        .map(|i| fix_labels(i, &labels))
        .collect();

    // Compiling
    // Translate symbols into binary code

    // Executing
    // Read memory line-by-line and interpret commands

    // Post-processing
    // Show logs

    for instruction in program.iter().map(|i| i.to_string()).collect::<Vec<_>>() {
        println!("{}", instruction);
    }
}

fn assemble(code: String) -> Vec<Instruction> {
    code.lines()
        .map(|l| l.split_once("--").unwrap_or((l, "")).0)
        .filter(|l| *l != "")
        .map(Instruction::new)
        .collect()
}

