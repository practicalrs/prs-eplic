use std::{error, fmt};

#[derive(Debug)]
pub enum InterpreterError {
    CommandLine,
    NoLoopEnd,
    UnexpectedLoopEnd,
}

impl error::Error for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InterpreterError!")
    }
}
