use crate::interp_err::InterpErr;
use std::{env, error::Error, fs::read};

mod brainfuck;
mod interp_err;
mod tp;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum Lng {
    Brainfuck,
    Unknown,
}

pub fn run() -> Result<()> {
    println!("Practical RS Esoteric Programming Languages Interpreters Collection.");

    let args: Vec<String> = env::args().collect();
    let file = args.last().ok_or(InterpErr::CmdLn)?;
    let ext = file.split('.').last();

    let lng = if let Some(ext) = ext {
        match ext {
            "b" | "bf" => Lng::Brainfuck,
            _ => Lng::Unknown,
        }
    } else {
        Lng::Unknown
    };

    let inpt = read(file)?;

    if let Lng::Brainfuck = lng {
        brainfuck::run(&inpt)?;
    }

    Ok(())
}
