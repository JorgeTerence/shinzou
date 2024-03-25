use std::{env, path};

use crate::quit;

pub struct Args {
    pub asm_path: String,
}

impl Args {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        let asm_path = match args.get(1) {
            Some(p) => {
                if !path::Path::new(p).exists() {
                    quit(&format!("Arquivo '{}' não existe.", p), 1);
                };
                p
            }
            None => quit("Código IAS auxente!", 1),
        };

        Self { asm_path: asm_path.to_string() }
    }
}
