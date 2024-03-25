mod cli;
mod ias;

use cli::{quit, warn, Args};
use ias::{Command, Instruction};
use std::collections::HashMap;
use std::fs;

use crate::ias::{Argument, Directive};

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
                // Jump blank lines
                if line == "" {
                    continue;
                }

                program.push(Instruction::new(line));
                // TODO: binary code
            }
        }
        Err(_) => quit("Erro ao ler arquivo do programa.", 1),
    }

    // Indexing
    // Set memory layout, clean-up symbols, leave only operators
    //
    // * Loop through everything replacing labels
    // * Check for missing labels
    // * Go line by line counting the memory position
    // * Use .org to set index of memory
    // * Associate labels to addresses
    // let mut memory: [Instruction; 2048];
    let mut counter = 0;
    let mut labels = HashMap::new();

    // TODO: .set values
    // Create label index
    for instruction in program.iter_mut() {
        if counter > 2048 {
            break; // TODO: warn about code that wasn't indexed
        }

        match &instruction.call {
            // Register new labels
            Command::Label(s) => {
                // Check if label is a duplicate
                match labels.insert(s, counter) {
                    None => (),
                    Some(old) => {
                        if old != counter {
                            warn(&format!(
                                "Duplicate label '{}' with values {} and {}",
                                s, old, counter
                            ));
                        }
                    }
                };
            }
            Command::Directive(dir) => match dir {
                Directive::Org => match &instruction.arg {
                    Argument::Addr(addr) => counter = *addr,
                    Argument::Label(_) => quit(
                        &format!(
                            ".org directives must use absolute values: '{}'",
                            instruction.to_string()
                        ),
                        1,
                    ),
                },
                Directive::Set => (),
                Directive::Word => (),
                Directive::Align => (),
                Directive::WFill => (),
            },
            _ => (),
        };

        counter += 1;
    }

    for instruction in program.iter_mut() {
        match &instruction.arg {
            Argument::Label(lbl) => {
                // Swap label for address
                instruction.arg = Argument::Addr(
                    *labels
                        .get(&lbl)
                        .unwrap_or_else(|| quit(&format!("Undeclared label '{}'", lbl), 1)),
                )
            }
            Argument::Addr(_) => (),
        }
    }

    // while counter < 2048 {
    //     counter += 1;
    // }

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
            .filter(|i| matches!(i.call, Command::Operator(_)))
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
    );
}
