use std::{env, fs::read_to_string, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let mut memory: Vec<Word> = vec![];
    // for _ in [0..1024] {
    //     memory.push(Word::new());
    // }

    let asm_path = match args.get(1) {
        Some(p) => {
            if !Path::new(p).exists() {
                quit(format!("Arquivo '{}' não existe.", p).as_str(), 1)
            }
            p
        }
        None => quit("IAS code not provided!", 1),
    };

    match read_to_string(asm_path) {
        Ok(program) => print!("{}", program),
        Err(_) => quit("Erro ao ler arquivo do programa.", 1),
    }
    let asm = read_to_string(asm_path).expect("file not found :(");
    println!("{}", asm);
}

fn quit(msg: &str, code: i32) -> ! {
    eprintln!("{}", msg);
    process::exit(code);
}

// struct Word {
//     op1: u8,
//     addr1: u16,
//     op2: u8,
//     addr2: u16,
// }

// impl Word {
//     fn new() -> Self {
//         Self {
//             op1: 0,
//             addr1: 0,
//             op2: 0,
//             addr2: 0,
//         }
//     }

// fn serialize(&self) -> String {
//     format!("{}{}{}{}", self.op1, self.addr1, self.op2, self.addr2)
// }

// fn repr() {}
// }
