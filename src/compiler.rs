use crate::cli::quit;
use crate::ias::{Argument, Command, Directive, Operator, Token};

#[derive(Debug)]
pub enum Sylable {
    Blank,
    Numeric(u16),
    Operator(u8, u16),
}

impl Sylable {
    fn from_assembly(op: Operator, arg: Argument) -> Self {
        Sylable::Operator(
            match op {
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
            },
            match arg {
                Argument::Addr(n) => n,
                Argument::Label(_) => quit(&format!("Invalid label at compile time: '{}'", arg), 1),
            },
        )
    }
}

impl std::fmt::Display for Sylable {
    /// Formats a sylable into a 20-bit binary string
    /// `111111111` := numeric value, `000000000` := blank, `*` := operator
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sylable::Numeric(n) => write!(f, "11111111{:012b}", n),
            Sylable::Operator(op, arg) => write!(f, "{:08b}{:012b}", op, arg),
            Sylable::Blank => write!(f, "{:020b}", 0),
        }
    }
}

/// Parses a token into a binary sequence
pub fn translate(token: Option<Token>) -> Sylable {
    match token {
        Some(i) => match i.call {
            // Operators are turn into their binary values
            Command::Operator(op) => Sylable::from_assembly(op, i.arg),
            // Try to populate .word addresses with numeric values
            Command::Directive(dir) => match dir {
                Directive::Word => match i.arg {
                    Argument::Addr(n) => Sylable::Numeric(n),
                    Argument::Label(_) => {
                        quit(&format!("Invalid label at compile time: '{}'", dir), 1)
                    }
                },
                _ => quit(&format!("Invalid directive at compile time: '{}'", dir), 1),
            },
            // Labels should not be present at compile time
            Command::Label(lbl) => quit(&format!("Invalid label at compile time: '{}'", lbl), 1),
        },
        None => Sylable::Blank,
    }
}
