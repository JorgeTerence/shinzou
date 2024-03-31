use std::collections::HashMap;

use crate::cli::{quit, warn};
use crate::ias::{Argument, Command, Directive, Token};

pub fn collect_definitions(program: &Vec<Token>) -> HashMap<String, u16> {
    let mut definitions = HashMap::new();

    for token in program.iter() {
        match &token.call {
            Command::Directive(dir) => match dir {
                Directive::Set => {
                    // Split the argument
                    let (lbl, value) = match &token.arg {
                        Argument::Label(lbl) => match lbl.split_once(' ') {
                            Some((s1, s2)) => (s1, s2),
                            None => quit(&format!("Invalid syntax for .set: {}", token), 1),
                        },
                        Argument::Addr(_) => {
                            quit(&format!("Invalid syntax for .set: {}", token), 1)
                        }
                    };

                    // Try to parse hex value
                    let addr = match u16::from_str_radix(value.trim_start_matches("0x"), 16) {
                        Ok(n) => n,
                        Err(_) => quit(
                            &format!(
                                ".org directives must use absolute values: '{}'",
                                token.to_string()
                            ),
                            1,
                        ),
                    };

                    // Check for duplicate definitions
                    match definitions.insert(lbl.to_string(), addr) {
                        Some(old) => {
                            if old != addr {
                                warn(&format!(
                                    "Duplicate label '{}' with values {} and {}",
                                    token, old, addr
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

pub fn collect_labels(program: &Vec<Token>) -> HashMap<String, u16> {
    let mut labels = HashMap::new();
    let mut counter = 0;

    for token in program.iter() {
        // TODO: 1024 word limit?
        match &token.call {
            Command::Label(s) => {
                // Check for duplicate labels
                match labels.insert(s.to_string(), counter) {
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

                continue;
            }
            _ => (),
        }

        counter += 1;
    }

    labels
}

/// Swap definitions' and labels' values for their addresses
pub fn link(token: Token, symbols: &HashMap<String, u16>) -> Token {
    Token {
        call: token.call,
        arg: match token.arg {
            Argument::Addr(_) => token.arg,
            Argument::Label(lbl) => Argument::Addr(
                *symbols
                    .get(&lbl)
                    .unwrap_or_else(|| quit(&format!("Undeclared symbol '{}'", lbl), 1)),
            ),
        },
    }
}

/// Arrange program according to `.org` directives.
/// Only leaves words and operators.
/// TODO: `.align` and `.wfill` directives.
pub fn allocate(program: Vec<Token>) -> Vec<Option<Token>> {
    let mut memory = vec![None; 2048];

    let mut counter = 0;

    for token in program.iter() {
        match &token.call {
            Command::Directive(dir) => match dir {
                Directive::Org => {
                    traverse(&mut counter, token);
                    continue;
                }
                _ => (),
            },
            _ => (),
        };

        memory[counter as usize] = Some(Token {
            call: token.call.clone(),
            arg: token.arg.clone(),
        });

        counter += 1;
    }

    memory
}

// Navigate memory position according to `.org` directives
fn traverse(counter: &mut u16, token: &Token) {
    match &token.arg {
        Argument::Addr(addr) => *counter = *addr,
        Argument::Label(_) => quit(
            &format!(
                ".org directives must use absolute values: '{}'",
                token.to_string()
            ),
            1,
        ),
    }
}

// TODO: expand .wfill and .align directives
