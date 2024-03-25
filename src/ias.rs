use crate::quit;

#[derive(Clone, Copy)]
enum Directive {
    Org,
    Set,
    Word,
    Align,
    WFill,
}

impl Directive {
    fn new(call: &str) -> Self {
        match call {
            "org" => Self::Org,
            "set" => Self::Set,
            "word" => Self::Word,
            "align" => Self::Align,
            "wfill" => Self::WFill,
            _ => quit(&format!("Diretiva '{}' não reconhecida.", call), 1),
        }
    }
}

#[derive(Clone, Copy)]
enum Operator {
    LoadFromMQ,     // LOAD MQ := AC <- MQ
    LoadMQ,         // LOAD MQ,M(X) := MQ <- M(X)
    LoadFromMemory, // LOAD M(X) := AC <- M(X)
    LoadNeg,        // LOAD -M(X) := AC <- -M(X)
    LoadAbs,        // LOAD |M(X)| := AC <- |M(X)|
    JumpLeft,       // JUMP M(X,0:19) := goto left of M(X)
    JumpRight,      // JUMP M(X,20:39) := goto right of M(X)
    JumpLeftIf,     // JUMP+M(X,0:19) := goto left of M(X) if AC >= 0
    JumpRightIf,    // JUMP+M(X,20:39) := goto right of M(X) if AC >= 0
    Add,            // ADD M(X) := AC <- AC + M(X)
    AddAbs,         // ADD |M(X)| := AC <- |AC + M(X)|
    Sub,            // SUB M(X) := AC <- AC - M(X)
    SubAbs,         // SUB |M(X)| := AC <- |AC - M(X)|
    Mul,            // MUL M(X) := AC, MQ <- MQ × M(X) 'AC contém os bits mais significativos'
    Div,            // DIV M(X) := MQ <- AC ÷ M(X), AC <- AC % M(X)
    Double,         // LSH := AC <- AC × 2 ou AC <- AC << 1
    Halve,          // RSH := AC <- AC ÷ 2 ou AC <- AC >> 1
    Store,          // STOR M(X) := M(X) <- AC ***
    StoreLeft,      // STOR M(X,8:19) := left of M(X) <- right of AC
    StoreRight,     // STOR M(X,28:39) := right of M(X) <- right of AC
}

// For the operations LSH, RSH and LOAD MQ, arg will be 0x00

union Command {
    dir: Directive,
    op: Operator,
}

union Argument {
    addr: u16,
    label: std::mem::ManuallyDrop<String>,
}

pub struct Instruction {
    call: Command, // 8 bits
    arg: Argument, // 12 bits
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        // Directive
        if line.starts_with(".") {
            let (call, arg) =  match line.split_once(' ') {
                Some((s1, s2)) => (Command { dir: Directive::new(s1) }, match  s2.parse::<u16>() {
                    Ok(n) => Argument {  },
                    Err(e) => quit(&format!("Erro na linha '{}'. Argumento acima de 12 bits. O valor máximo é {}", line, u16::pow(2, 16)), 1)
                }),
                None => quit(&format!("Diretiva mal formatada: {}", line), 1)
            };

            return Self { call, arg };
        }
        // Rótulos
        else if line.ends_with(":") {
            Self {}
        }
        // Operation
        else {
            Self {
                call: Command {
                    op: Operator::LoadFromMemory,
                },
            }
        }
    }
}

// TODO: words (only make sense in binary)
