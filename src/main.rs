mod cli;
mod ias;

use cli::{quit, warn, Args};
use ias::{Command, Instruction};
use std::collections::HashMap;
use std::fs;

use crate::ias::{Argument, Directive};

fn main() {
    let args = Args::new();

    let mut program: Vec<Instruction>;

    // Assembling
    // Transform strings into symbols -> directives, labels and operators
    match fs::read_to_string(args.asm_path) {
        Ok(code) => program = assemble(code),
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
    // TODO: .set values before labels
    let definititions = collect_definitions(program.clone());

    let labels = collect_labels(program.clone());

    // create swap addresses function (vec, hashmap) -> vec
    for instruction in program.iter_mut() {
        match &instruction.arg {
            Argument::Label(lbl) => {
                // Swap label for address
                instruction.arg = Argument::Addr(
                    *labels
                        .get(lbl)
                        .unwrap_or_else(|| quit(&format!("Undeclared label '{}'", lbl), 1)),
                )
            }
            Argument::Addr(_) => (),
        }
    }

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

fn assemble(code: String) -> Vec<Instruction> {
    code.lines()
        .map(|l| l.split_once("--").unwrap_or((l, "")).0)
        .filter(|l| *l != "")
        .map(Instruction::new)
        .collect()
}

fn collect_definitions(program: Vec<Instruction>) -> HashMap<String, u16> {
    let mut definitions = HashMap::new();
    let mut counter = 0;

    for instruction in program.iter() {
        // TODO: 1024 word limit?
        match &instruction.call {
            // Navigate memory
            Command::Directive(dir) => match dir {
                Directive::Set => {
                    println!("{:?}", instruction.arg);
                    // match definitions.insert(instruction.arg, counter) {
                    //     // arg contains key and value
                    //     // Check for duplicate definitions
                    //     Some(old) => {
                    //         if old != counter {
                    //             warn(&format!(
                    //                 "Duplicate label '{}' with values {} and {}",
                    //                 s, old, counter
                    //             ));
                    //         }
                    //     }
                    //     None => (),
                    // }
                }
                // Directive::Org => match &instruction.arg {
                //     Argument::Addr(addr) => counter = *addr,
                //     // Warn about labels in .org directives
                //     Argument::Label(_) => quit(
                //         &format!(
                //             ".org directives must use absolute values: '{}'",
                //             instruction.to_string()
                //         ),
                //         1,
                //     ),
                // },
                _ => (),
            },
            _ => (),
        }

        counter += 1;
    }

    definitions
}

fn collect_labels(program: Vec<Instruction>) -> HashMap<String, u16> {
    let mut labels = HashMap::new();
    let mut counter = 0;

    for instruction in program.iter() {
        // TODO: 1024 word limit?
        match &instruction.call {
            Command::Label(s) => {
                match labels.insert(s.to_string(), counter) {
                    // Check for duplicate labels
                    Some(old) => {
                        if old != counter {
                            warn(&format!(
                                "Duplicate label '{}' with values {} and {}",
                                s, old, counter
                            ));
                        }
                    }
                    None => (),
                };
            }

            // Navigate memory
            Command::Directive(dir) => match dir {
                Directive::Org => match &instruction.arg {
                    Argument::Addr(addr) => counter = *addr,
                    // Warn about labels in .org directives
                    Argument::Label(_) => quit(
                        &format!(
                            ".org directives must use absolute values: '{}'",
                            instruction.to_string()
                        ),
                        1,
                    ),
                },
                _ => (),
            },
            _ => (),
        }

        counter += 1;
    }

    labels
}
