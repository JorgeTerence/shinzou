use crate::{
    cli::quit,
    ias::{Argument, Command, Directive, Instruction, Operator},
};

#[derive(Debug)]
pub enum Sylable {
    Numeric(u16),
    Operator(u8), // NEXT: add argument to call (12 bits)
}

impl Sylable {
    fn from_assembly(op: Operator) -> Self {
        Sylable::Operator(match op {
            Operator::LoadFromMQ => 0b0000_1010,
            Operator::LoadMQ => 0b0000_1001,
            Operator::LoadFromMemory => 0b0000_01,
            Operator::LoadNeg => 0b0000_0010,
            Operator::LoadAbs => 0b0000_0011,
            Operator::JumpLeft => 0b0000_1101,
            Operator::JumpRight => 0b0000_1110,
            Operator::JumpLeftIf => 0b0000_1111,
            Operator::JumpRightIf => 0b0001_0000,
            Operator::Add => 0b0000_0101,
            Operator::AddAbs => 0b0000_0111,
            Operator::Sub => 0b0000_0110,
            Operator::SubAbs => 0b0000_1000,
            Operator::Mul => 0b0000_1011,
            Operator::Div => 0b0000_1100,
            Operator::Double => 0b0001_0100,
            Operator::Halve => 0b0001_0101,
            Operator::Store => 0b0010_0001,
            Operator::StoreLeft => 0b0001_0010,
            Operator::StoreRight => 0b0001_0011,
            Operator::Output => 0b1000_0001,
            Operator::Character => 0b1000_0010,
        })
    }
}

impl std::fmt::Display for Sylable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sylable::Numeric(n) => write!(f, "{:012b}", n),
            Sylable::Operator(op) => write!(f, "{:08b}", op),
        }
    }
}

pub fn translate(instruction: Option<Instruction>) -> Option<Sylable> {
    match instruction {
        Some(i) => match i.call {
            // Operators are turn into their binary values
            Command::Operator(op) => Some(Sylable::from_assembly(op)),
            // Try to populate .word addresses with numeric values
            Command::Directive(dir) => match dir {
                Directive::Word => match i.arg {
                    Argument::Addr(n) => Some(Sylable::Numeric(n)),
                    Argument::Label(_) => {
                        quit(&format!("Invalid label at compile time: '{}'", dir), 1)
                    }
                },
                _ => quit(&format!("Invalid directive at compile time: '{}'", dir), 1),
            },
            // Labels should not be present at compile time
            Command::Label(lbl) => quit(&format!("Invalid label at compile time: '{}'", lbl), 1),
        },
        None => None,
    }
}
