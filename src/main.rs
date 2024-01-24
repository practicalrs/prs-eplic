#![forbid(unsafe_code)]

/// Please forgive me for using all these abbreviations all over the code.
/// I just wanted to check how it feels to use Rust like my favorite
/// macro assembler - C language ;)
use crate::interpreter_error::InterpreterError;
use std::{env, error::Error, fs::read};

mod brainfuck;
mod interpreter_error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum Lng {
    Brainfuck,
    Unknown,
}

fn main() -> Result<()> {
    println!("Practical RS Esoteric Programming Languages Interpreters Collection.");

    let args: Vec<String> = env::args().collect();
    let file = args.last().ok_or(InterpreterError::CommandLine)?;
    let ext = file.split('.').last();

    let lng = if let Some(ext) = ext {
        match ext {
            "bf" => Lng::Brainfuck,
            _ => Lng::Unknown,
        }
    } else {
        Lng::Unknown
    };

    let inpt = read(file)?;

    if let Lng::Brainfuck = lng {
        let _ = brainfuck::run(&inpt);
    }

    Ok(())
}
