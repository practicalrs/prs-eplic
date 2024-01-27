use std::{error, fmt};

#[derive(Debug)]
pub enum InterpErr {
    AddOvflw,
    CmdLn,
    NoLpEn,
    SubOvflw,
    UnexpInst,
    UnexpLpEn,
}

impl error::Error for InterpErr {}

impl fmt::Display for InterpErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InterpErr!")
    }
}
