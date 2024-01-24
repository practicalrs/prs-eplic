/// Heavily inspired by:
/// <https://github.com/nushell/nushell/blob/main/crates/nu-parser/src/lex.rs>
/// <https://github.com/Overv/bf/blob/master/src/main.rs>
use crate::{interpreter_error::InterpreterError, Result};
use std::io::{stdin, Read};

#[derive(Debug)]
enum Cmd {
    Dec,
    DecPtr,
    Inc,
    IncPtr,
    Lp(Vec<Cmd>),
    LpBg,
    LpEn,
    Ld,
    St,
}

fn exec(cmds: &[Cmd], mem: &mut [u8], ptr: &mut usize) -> Result<()> {
    //    println!("ptr = {ptr:?}");
    for cmd in cmds {
        match cmd {
            Cmd::Dec => mem[*ptr] -= 1,
            Cmd::DecPtr => *ptr -= 1,
            Cmd::Inc => mem[*ptr] += 1,
            Cmd::IncPtr => *ptr += 1,
            Cmd::Ld => print!("{}", mem[*ptr] as char),
            Cmd::Lp(cmds) => {
                while mem[*ptr] != 0 {
                    exec(cmds, mem, ptr)?;
                }
            }
            Cmd::St => {
                let mut inpt = [0; 1];
                stdin().read_exact(&mut inpt)?;
                mem[*ptr] = inpt[0];
            }
            _ => {}
        }
    }

    Ok(())
}

fn lex(inpt: &[u8]) -> Vec<Cmd> {
    let mut cmds = vec![];
    let mut off = 0;

    while let Some(c) = inpt.get(off) {
        let cmd = match c {
            b'>' => Some(Cmd::IncPtr),
            b'<' => Some(Cmd::DecPtr),
            b'+' => Some(Cmd::Inc),
            b'-' => Some(Cmd::Dec),
            b'.' => Some(Cmd::Ld),
            b',' => Some(Cmd::St),
            b'[' => Some(Cmd::LpBg),
            b']' => Some(Cmd::LpEn),
            _ => None,
        };

        if let Some(cmd) = cmd {
            cmds.push(cmd);
        }

        off += 1;
    }

    cmds
}

fn parse(cmds: &[Cmd]) -> Result<Vec<Cmd>> {
    let mut parsed_cmds = vec![];
    let mut lp_bg = 0;
    let mut lp_stck = 0;

    for (i, cmd) in cmds.iter().enumerate() {
        if lp_stck == 0 {
            let parsed_cmd = match cmd {
                Cmd::Dec => Some(Cmd::Dec),
                Cmd::DecPtr => Some(Cmd::DecPtr),
                Cmd::Inc => Some(Cmd::Inc),
                Cmd::IncPtr => Some(Cmd::IncPtr),
                Cmd::Ld => Some(Cmd::Ld),
                Cmd::Lp(_) => None,
                Cmd::LpBg => {
                    lp_bg = i;
                    lp_stck += 1;
                    None
                }
                Cmd::LpEn => {
                    return Err(Box::new(InterpreterError::UnexpectedLoopEnd));
                }
                Cmd::St => Some(Cmd::St),
            };

            if let Some(parsed_cmd) = parsed_cmd {
                parsed_cmds.push(parsed_cmd);
            }
        } else {
            match cmd {
                Cmd::LpBg => {
                    lp_stck += 1;
                }
                Cmd::LpEn => {
                    lp_stck -= 1;

                    if lp_stck == 0 {
                        parsed_cmds.push(Cmd::Lp(parse(&cmds[lp_bg + 1..i])?));
                    }
                }
                _ => {}
            }
        }
    }

    if lp_stck != 0 {
        return Err(Box::new(InterpreterError::NoLoopEnd));
    }

    Ok(parsed_cmds)
}

pub fn run(inpt: &[u8]) -> Result<()> {
    let cmds = lex(inpt);
    let parsed_cmds = parse(&cmds)?;

    let mut mem = vec![0; 2048];
    let mut ptr = 1024;

    exec(&parsed_cmds, &mut mem, &mut ptr)?;

    //println!("MEM = {mem:?}");

    Ok(())
}
