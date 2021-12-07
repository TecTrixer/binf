use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BfError {
    InvalidProgram { invalid_char: char },
    RuntimeError,
    NoInput,
    InvalidProgramBrackets { unmatched: usize },
}

impl Error for BfError {}

impl std::fmt::Display for BfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BfError::InvalidProgram { invalid_char: x } => {
                write!(f, "the character {:?} is invalid in a Brainfuck program", x)
            }
            BfError::RuntimeError => {
                write!(
                    f,
                    "there was a runtime error during the execution of your program"
                )
            }
            BfError::NoInput => {
                write!(f, "your program expected input but there was none left")
            }
            BfError::InvalidProgramBrackets { unmatched: x } => {
                write!(f, "your program has an unmatched bracket at position {}", x)
            }
        }
    }
}
