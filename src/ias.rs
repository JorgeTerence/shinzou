use core::fmt;

use crate::quit;

#[derive(Clone, Copy, Debug)]
pub enum Directive {
    Org,
    Set,
    Word,
    Align,
    WFill,
}

impl Directive {
    fn new(call: &str) -> Self {
        match call {
            ".org" => Self::Org,
            ".set" => Self::Set,
            ".word" => Self::Word,
            ".align" => Self::Align,
            ".wfill" => Self::WFill,
            _ => quit(&format!("Diretiva '{}' não reconhecida.", call), 1),
        }
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Org => ".org",
            Self::Set => ".set",
            Self::Word => ".word",
            Self::Align => ".align",
            Self::WFill => ".wfill",
        };
        write!(f, "{}", label)
    }
}

#[derive(Clone, Copy, Debug)]
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

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::LoadFromMQ => "LOAD MQ",
            Self::LoadMQ => "LOAD MQ,M(X)",
            Self::LoadFromMemory => "LOAD M(X)",
            Self::LoadNeg => "LOAD -M(X)",
            Self::LoadAbs => "LOAD |M(X)|",
            Self::JumpLeft => "JUMP M(X,0:19)",
            Self::JumpRight => "JUMP M(X,20:39)",
            Self::JumpLeftIf => "JUMP+M(X,0:19)",
            Self::JumpRightIf => "JUMP+M(X,20:39)",
            Self::Add => "ADD M(X)",
            Self::AddAbs => "ADD |M(X)|",
            Self::Sub => "SUB M(X)",
            Self::SubAbs => "SUB |M(X)|",
            Self::Mul => "MUL M(X)",
            Self::Div => "DIV M(X)",
            Self::Double => "LSH",
            Self::Halve => "RSH",
            Self::Store => "STOR M(X)",
            Self::StoreLeft => "STOR M(X,8:19)",
            Self::StoreRight => "STOR M(X,28:39)",
        };

        write!(f, "{}", label)
    }
}

// For the operations LSH, RSH and LOAD MQ, arg will be 0x00

#[derive(Debug)]
pub enum Command {
    Directive(Directive),
    Operator(Operator),
    Label(String),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Directive(dir) => dir.to_string(),
            Self::Operator(op) => op.to_string(),
            Self::Label(s) => s.to_string(),
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug)]
pub enum Argument {
    Addr(u16),
    Label(String),
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Addr(n) => n.to_string(),
            Self::Label(l) => l.to_string()
        };
        write!(f, "{}", label)
    }
}

pub struct Instruction {
    pub call: Command, // 8 bits
    pub arg: Argument, // 12 bits
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        // Directive
        if line.starts_with(".") {
            match line.split_once(' ') {
                Some((s1, s2)) => {
                    let call = Command::Directive(Directive::new(s1));
                    let arg = match s2.parse::<u16>() {
                        Ok(n) => Argument::Addr(n),
                        Err(e) => Argument::Label(s2.to_string()),
                    };
                    return Self { call, arg };
                }
                None => quit(&format!("Diretiva mal formatada: {}", line), 1),
            };
        }
        // Rótulos
        else if line.ends_with(":") {
            Self {
                call: Command::Operator(Operator::Add),
                arg: Argument::Addr(0x011),
            }
        }
        // Operation
        else {
            Self {
                call: Command::Label("Hello world".to_string()),
                arg: Argument::Addr(0x011),
            }
        }
    }
}

// TODO: words (only make sense in binary)
