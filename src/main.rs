mod cli;
mod compiler;
mod ias;
mod index;
mod runtime;

use crate::cli::{quit, Args};
use crate::compiler::{translate, Sylable};
use crate::ias::{Command, Directive, Token};
use crate::index::{allocate, collect_definitions, collect_labels, link};
use crate::runtime::run;
use clap::Parser;
use std::{collections::HashMap, fs};

fn main() {
    let args = Args::parse();

    match args {
        Args::Run { path } => {
            // Assembling
            // Transform strings into symbols -> directives, labels, operators
            // TODO: Check if it's a valid file
            // TODO: Check if it's an IAS file or memory map
            let code = match fs::read_to_string(path) {
                Ok(code) => code,
                Err(e) => quit(&format!("Error reading the program: {}", e), 1),
            };

            let mut tokens: Vec<Token> = code
                .lines()
                // Remove comments
                .map(|l| l.split_once("--").unwrap_or((l, "")).0)
                // Remove empty lines
                .filter(|l| *l != "")
                .map(Token::new)
                .collect();

            // Indexing
            // Set memory layout, clean-up symbols, leave only operators
            let definititions = collect_definitions(&tokens);
            tokens.retain(|t| !matches!(t.call, Command::Directive(Directive::Set)));

            let labels = collect_labels(&tokens);
            tokens.retain(|t| !matches!(t.call, Command::Label(_)));

            let mut symbols = HashMap::new();
            symbols.extend(definititions);
            symbols.extend(labels);

            tokens = tokens.into_iter().map(|t| link(t, &symbols)).collect();

            let program = allocate(tokens);

            // Compiling
            // TODO: Warn about overwritten memory
            let sylables: Vec<Sylable> = program.into_iter().map(translate).collect();
            // TODO: Pad last word with zeroes if the count is odd
            let pairs = sylables.chunks_exact(2);
            let mut executable = pairs
                .clone()
                .map(|s| format!("{}{}{}", s[0], s[1], '\n'))
                .collect::<String>();

            if pairs.remainder().len() > 0 {
                executable.push_str(&pairs.remainder()[0].to_string());
                executable.push_str("00000000000000000000\n"); // TODO: Test if this works
            }

            // Executing
            // Read memory line-by-line and interpret commands
            run(executable);

            // Post-processing
            // Show logs
        }
        _ => unimplemented!(),
        // Args::RunBin { bin_path } => run_bin(bin_path),
        // Args::Compile { asm_path, bin_path } => compile(asm_path, bin_path),
    }
}

