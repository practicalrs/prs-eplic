/// Heavily inspired by:
/// <https://esolangs.org/wiki/Sus>
/// <https://github.com/iG-Studios/sus>
use crate::{tp::Tp, Result};

#[derive(Debug)]
enum Cmd {
    Dec,
    DecPtr,
    Inc,
    IncPtr,
    Ld,
}

fn exec(cmds: &[Cmd], tp: &mut Tp) -> Result<()> {
    for cmd in cmds {
        match cmd {
            Cmd::Dec => tp.dec()?,
            Cmd::DecPtr => tp.dec_ptr()?,
            Cmd::Inc => tp.inc()?,
            Cmd::IncPtr => tp.inc_ptr()?,
            Cmd::Ld => tp.print()?,
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
            b'|' => Some(Cmd::Ld),
            _ => None,
        };

        if let Some(cmd) = cmd {
            cmds.push(cmd);
        }

        off += 1;
    }

    cmds
}

pub fn run(inpt: &[u8]) -> Result<()> {
    let cmds = lex(inpt);
    println!("CMDS = {:?}", cmds);

    let mut tp = Tp::new();

    exec(&cmds, &mut tp)?;

    Ok(())
}
