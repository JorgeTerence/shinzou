use std::collections::HashMap;

use crate::cli::{quit, warn};
use crate::ias::{Argument, Command, Directive, Instruction};

pub fn collect_definitions(program: Vec<Instruction>) -> HashMap<String, u16> {
    let mut definitions = HashMap::new();

    for instruction in program.iter() {
        match &instruction.call {
            Command::Directive(dir) => match dir {
                Directive::Set => {
                    let (lbl, value) = match &instruction.arg {
                        //split the argument
                        Argument::Label(lbl) => match lbl.split_once(' ') {
                            Some((s1, s2)) => (s1, s2),
                            None => quit(&format!("Invalid syntax for .set: {}", instruction), 1),
                        },
                        Argument::Addr(_) => {
                            quit(&format!("Invalid syntax for .set: {}", instruction), 1)
                        }
                    };

                    let addr = match u16::from_str_radix(value.trim_start_matches("0x"), 16) {
                        Ok(n) => n,
                        Err(_) => quit(
                            &format!(
                                ".org directives must use absolute values: '{}'",
                                instruction.to_string()
                            ),
                            1,
                        ),
                    };

                    match definitions.insert(lbl.to_string(), addr) {
                        // Check for duplicate definitions
                        Some(old) => {
                            if old != addr {
                                warn(&format!(
                                    "Duplicate label '{}' with values {} and {}",
                                    instruction, old, addr
                                ));
                            }
                        }
                        None => (),
                    };
                }
                _ => (),
            },
            _ => (),
        }
    }

    definitions
}

pub fn collect_labels(program: Vec<Instruction>) -> HashMap<String, u16> {
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
            // TODO: create function to traverse counter
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

pub fn fix_symbols(instruction: Instruction, symbols: &HashMap<String, u16>) -> Instruction {
    Instruction {
        call: instruction.call,
        arg: match instruction.arg {
            Argument::Addr(_) => instruction.arg,
            Argument::Label(lbl) => Argument::Addr(
                *symbols
                    .get(&lbl)
                    .unwrap_or_else(|| quit(&format!("Undeclared label '{}'", lbl), 1)),
            ),
        },
    }
}
