/// Heavily inspired by:
/// <https://github.com/nushell/nushell>
/// <https://github.com/Overv/bf>
/// <https://github.com/tov/bf-rs>
/// <https://github.com/Wilfred/bfc>
/// <https://github.com/benkonz/brainfrick-rust>
/// <https://github.com/nixpulvis/brainfuck>
use crate::{interp_err::InterpErr, tp::Tp, Result};

#[derive(Debug, PartialEq)]
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

fn exec(cmds: &[Cmd], tp: &mut Tp) -> Result<()> {
    let debug = false;
    for cmd in cmds {
        if debug {
            println!("cmd = {cmd:?}, tpdbg = {:?}", tp.dbg());
        }
        match cmd {
            Cmd::Dec => tp.dec()?,
            Cmd::DecPtr => tp.dec_ptr()?,
            Cmd::Inc => tp.inc()?,
            Cmd::IncPtr => tp.inc_ptr()?,
            Cmd::Ld => tp.print()?,
            Cmd::Lp(cmds) => {
                while tp.get() != 0 {
                    exec(cmds, tp)?;
                }
            }
            Cmd::LpBg | Cmd::LpEn => {
                return Err(Box::new(InterpErr::UnexpInst));
            }
            Cmd::St => tp.read()?,
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
                Cmd::Lp(_) => {
                    return Err(Box::new(InterpErr::UnexpInst));
                }
                Cmd::LpBg => {
                    lp_bg = i;
                    lp_stck += 1;
                    None
                }
                Cmd::LpEn => {
                    return Err(Box::new(InterpErr::UnexpLpEn));
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
        return Err(Box::new(InterpErr::NoLpEn));
    }

    Ok(parsed_cmds)
}

pub fn run(inpt: &[u8]) -> Result<()> {
    let cmds = lex(inpt);
    let parsed_cmds = parse(&cmds)?;

    let mut tp = Tp::new();

    exec(&parsed_cmds, &mut tp)?;

    //println!("MEM = {mem:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::brainfuck::{lex, parse, Cmd};

    #[test]
    fn lex_single_cmds() {
        let inpt = ">".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![Cmd::IncPtr]);

        let inpt = "<".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![Cmd::DecPtr]);

        let inpt = "+".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![Cmd::Inc]);

        let inpt = "-".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![Cmd::Dec]);

        let inpt = ".".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![Cmd::Ld]);

        let inpt = ",".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![Cmd::St]);

        let inpt = "[".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![Cmd::LpBg]);

        let inpt = "]".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![Cmd::LpEn]);
    }

    #[test]
    fn lex_multi_cmds() {
        let inpt = "><+-.,[]".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(
            res,
            vec![
                Cmd::IncPtr,
                Cmd::DecPtr,
                Cmd::Inc,
                Cmd::Dec,
                Cmd::Ld,
                Cmd::St,
                Cmd::LpBg,
                Cmd::LpEn
            ]
        );

        let inpt = "][,.-+<>".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(
            res,
            vec![
                Cmd::LpEn,
                Cmd::LpBg,
                Cmd::St,
                Cmd::Ld,
                Cmd::Dec,
                Cmd::Inc,
                Cmd::DecPtr,
                Cmd::IncPtr,
            ]
        );

        let inpt = "ABCDEFGHIJKLMNOPRSTUWXYZ".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![]);

        let inpt = "abcdefghijklmnoprstuwxyz".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![]);

        let inpt = "1234567890".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(res, vec![]);

        let inpt = "a>b<c+d-e.f,g[h]i".as_bytes();
        let res = lex(inpt);

        debug_assert_eq!(
            res,
            vec![
                Cmd::IncPtr,
                Cmd::DecPtr,
                Cmd::Inc,
                Cmd::Dec,
                Cmd::Ld,
                Cmd::St,
                Cmd::LpBg,
                Cmd::LpEn
            ]
        );
    }

    #[test]
    fn parse_single_cmds() {
        let cmds = vec![Cmd::IncPtr];
        let res = parse(&cmds).unwrap();

        debug_assert_eq!(res, vec![Cmd::IncPtr]);

        let cmds = vec![Cmd::DecPtr];
        let res = parse(&cmds).unwrap();

        debug_assert_eq!(res, vec![Cmd::DecPtr]);

        let cmds = vec![Cmd::Inc];
        let res = parse(&cmds).unwrap();

        debug_assert_eq!(res, vec![Cmd::Inc]);

        let cmds = vec![Cmd::Dec];
        let res = parse(&cmds).unwrap();

        debug_assert_eq!(res, vec![Cmd::Dec]);

        let cmds = vec![Cmd::Ld];
        let res = parse(&cmds).unwrap();

        debug_assert_eq!(res, vec![Cmd::Ld]);

        let cmds = vec![Cmd::St];
        let res = parse(&cmds).unwrap();

        debug_assert_eq!(res, vec![Cmd::St]);

        let cmds = vec![Cmd::LpBg, Cmd::LpEn];
        let res = parse(&cmds).unwrap();

        debug_assert_eq!(res, vec![Cmd::Lp(vec![])]);
    }

    #[test]
    fn parse_multi_cmds() {
        let cmds = vec![
            Cmd::IncPtr,
            Cmd::DecPtr,
            Cmd::Inc,
            Cmd::Dec,
            Cmd::Ld,
            Cmd::St,
            Cmd::LpBg,
            Cmd::LpEn,
        ];
        let res = parse(&cmds).unwrap();

        debug_assert_eq!(
            res,
            vec![
                Cmd::IncPtr,
                Cmd::DecPtr,
                Cmd::Inc,
                Cmd::Dec,
                Cmd::Ld,
                Cmd::St,
                Cmd::Lp(vec![]),
            ]
        );

        let cmds = vec![Cmd::Inc, Cmd::LpBg, Cmd::Inc, Cmd::LpEn, Cmd::Inc];
        let res = parse(&cmds).unwrap();

        assert_eq!(res, vec![Cmd::Inc, Cmd::Lp(vec![Cmd::Inc]), Cmd::Inc]);

        let cmds = vec![
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
        ];
        let res = parse(&cmds).unwrap();

        assert_eq!(
            res,
            vec![
                Cmd::Inc,
                Cmd::Lp(vec![Cmd::Inc, Cmd::Lp(vec![Cmd::Inc]), Cmd::Inc]),
                Cmd::Inc
            ]
        );

        let cmds = vec![
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
        ];
        let res = parse(&cmds).unwrap();

        assert_eq!(
            res,
            vec![
                Cmd::Inc,
                Cmd::Lp(vec![
                    Cmd::Inc,
                    Cmd::Lp(vec![Cmd::Inc]),
                    Cmd::Lp(vec![Cmd::Inc]),
                    Cmd::Inc
                ]),
                Cmd::Inc
            ]
        );

        let cmds = vec![
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
        ];
        let res = parse(&cmds).unwrap();

        assert_eq!(
            res,
            vec![
                Cmd::Inc,
                Cmd::Lp(vec![
                    Cmd::Inc,
                    Cmd::Lp(vec![Cmd::Inc, Cmd::Lp(vec![Cmd::Inc]), Cmd::Inc]),
                    Cmd::Lp(vec![Cmd::Inc, Cmd::Lp(vec![Cmd::Inc]), Cmd::Inc]),
                    Cmd::Inc
                ]),
                Cmd::Inc
            ]
        );

        let cmds = vec![
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpBg,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
            Cmd::LpEn,
            Cmd::Inc,
        ];
        let res = parse(&cmds).unwrap();

        assert_eq!(
            res,
            vec![
                Cmd::Inc,
                Cmd::Lp(vec![
                    Cmd::Inc,
                    Cmd::Lp(vec![Cmd::Inc, Cmd::Lp(vec![Cmd::Inc]), Cmd::Inc]),
                    Cmd::Lp(vec![Cmd::Inc, Cmd::Lp(vec![Cmd::Inc]), Cmd::Inc]),
                    Cmd::Inc
                ]),
                Cmd::Inc,
                Cmd::Inc,
                Cmd::Lp(vec![
                    Cmd::Inc,
                    Cmd::Lp(vec![Cmd::Inc, Cmd::Lp(vec![Cmd::Inc]), Cmd::Inc]),
                    Cmd::Lp(vec![Cmd::Inc, Cmd::Lp(vec![Cmd::Inc]), Cmd::Inc]),
                    Cmd::Inc
                ]),
                Cmd::Inc
            ]
        );
    }
}
