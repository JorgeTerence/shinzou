use core::fmt;
use regex::{Regex, RegexSet};

use crate::quit;

const OPERATORS_MATCHES: [&str; 22] = [
    // Data transfer
    r"load\s+mq",
    r"load\s+mq,m\(.+\)",
    r"load\s+m\(.+\)",
    r"load\s+-m\(.+\)",
    r"load\s+\|m\(.+\)\|",
    // Jumps
    r"jump\s+m\(.+,\s*0:19\)",
    r"jump\s+m\(.+,\s*20:39\)",
    r"jump\+\s*m\(.+,\s*0:19\)",
    r"jump\+\s*m\(.+,\s*20:39\)",
    // Math
    r"add\s+m\(.+\)",
    r"add\s+m\|\(.+\)\|",
    r"sub\s+m\(.+\)",
    r"sub\s+m\|\(.+\)\|",
    r"mul\s+m\(.+\)",
    r"div\s+m\(.+\)",
    r"lsh",
    r"rsh",
    // Storage
    r"stor\s+m\(.+\)",
    r"stor\s+m\(.+\,\s*8:19\)",
    r"stor\s+m\(.+\,\s*28:39\)",
    // I/O
    r"out\s+m\(.+\)",
    r"char\s+m\(.+\)",
];

const OPERATORS: [Operator; 22] = [
    Operator::LoadFromMQ,
    Operator::LoadMQ,
    Operator::LoadFromMemory,
    Operator::LoadNeg,
    Operator::LoadAbs,
    Operator::JumpLeft,
    Operator::JumpRight,
    Operator::JumpLeftIf,
    Operator::JumpRightIf,
    Operator::Add,
    Operator::AddAbs,
    Operator::Sub,
    Operator::SubAbs,
    Operator::Mul,
    Operator::Div,
    Operator::Double,
    Operator::Halve,
    Operator::Store,
    Operator::StoreLeft,
    Operator::StoreRight,
    Operator::Output,
    Operator::Character,
];

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
            Self::Org => ".org %",
            Self::Set => ".set %",
            Self::Word => ".word %",
            Self::Align => ".align %",
            Self::WFill => ".wfill %",
        };
        write!(f, "{}", label)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    LoadFromMQ,     // LOAD MQ := AC <- MQ
    LoadMQ,         // LOAD MQ,M(X) := MQ <- M(X)
    LoadFromMemory, // LOAD M(X) := AC <- M(X)
    LoadNeg,        // LOAD -M(X) := AC <- -M(X)
    LoadAbs,        // LOAD |M(X)| := AC <- |M(X)|
    JumpLeft,       // JUMP M(X,0:19) := goto left of M(X)
    JumpRight,      // JUMP M(X,20:39) := goto right of M(X)
    JumpLeftIf,     // JUMP+ M(X,0:19) := goto left of M(X) if AC >= 0
    JumpRightIf,    // JUMP+ M(X,20:39) := goto right of M(X) if AC >= 0
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
    Output,         // OUT M(X) := I/O <- M(X)
    Character,      // CHAR M(X) := I/O <- ASCII(M(X))
}

impl Operator {
    fn new(call: &str) -> Self {
        let call = call.to_lowercase();

        let regex_set = RegexSet::new(&OPERATORS_MATCHES).unwrap();
        let nails: Vec<usize> = regex_set.matches(&call).into_iter().collect();

        if nails.len() == 0 {
            quit(&format!("Unindentified operator: {}", call), 1);
        }

        OPERATORS[nails[0]]
    }
}

impl fmt::Display for Operator {
    // TODO: Fix this -> remove arguments from representation
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::LoadFromMQ => "load mq",
            Self::LoadMQ => "load mq,m(%)",
            Self::LoadFromMemory => "load m(%)",
            Self::LoadNeg => "load -m(%)",
            Self::LoadAbs => "load |m(%)",
            Self::JumpLeft => "jump m(%,0:19)",
            Self::JumpRight => "jump m(%,20:39)",
            Self::JumpLeftIf => "jump+ m(%,0:19)",
            Self::JumpRightIf => "jump+ m(%,20:39)",
            Self::Add => "add m(%)",
            Self::AddAbs => "add |m(%)|",
            Self::Sub => "sub m(%)",
            Self::SubAbs => "sub |m(%)|",
            Self::Mul => "mul m(%)",
            Self::Div => "div m(%)",
            Self::Double => "lsh",
            Self::Halve => "lsh",
            Self::Store => "stor m(%)",
            Self::StoreLeft => "stor m(%,8:19)",
            Self::StoreRight => "stor m(%,28:39)",
            Self::Output => "out m(%)",
            Self::Character => "char m(%)",
        };

        write!(f, "{}", label)
    }
}

// For the operations LSH, RSH and LOAD MQ, arg will be 0x00

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Argument {
    Addr(u16),
    Label(String),
}

impl Argument {
    fn new(arg: &str) -> Self {
        match u16::from_str_radix(arg.trim_start_matches("0x"), 16) {
            Ok(n) => Argument::Addr(n), // TODO: Limit numbers to 2^12
            Err(_) => Argument::Label(arg.to_string()),
        }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Addr(n) => n.to_string(),
            Self::Label(l) => l.to_string(),
        };
        write!(f, "{}", label)
    }
}

#[derive(Debug, Clone)]
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
                    return Self {
                        call: Command::Directive(Directive::new(s1)),
                        arg: Argument::new(s2),
                    };
                }
                None => quit(&format!("Diretiva mal formatada: {}", line), 1),
            };
        }
        // Rótulos
        else if line.ends_with(":") {
            Self {
                call: Command::Label(line.replace(':', "")),
                arg: Argument::Addr(0),
            }
        }
        // Operation
        else {
            let arg_finder = Regex::new(r"m\(.+\)").unwrap();
            let escape_call = Regex::new(r"(m|\(|\)|0:19|20:39|8:19|28:39|,)*").unwrap();

            let op = Operator::new(line);
            let arg = match op {
                Operator::LoadFromMQ | Operator::Double | Operator::Halve => Argument::Addr(0),
                _ => match arg_finder.find(line) {
                    Some(value) => {
                        // Find the X in M(X)
                        Argument::new(escape_call.replace_all(value.into(), "").as_ref())
                    }
                    None => quit(&format!("Poorly formated argument: {}", line), 1),
                },
            };
            Self {
                call: Command::Operator(op),
                arg,
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.call.to_string().replace("%", &self.arg.to_string()))
    }
}

// TODO: words (only make sense in binary)
