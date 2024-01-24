#![forbid(unsafe_code)]

/// Please forgive me for using all these abbreviations all over the code.
/// I just wanted to check how it feels to use Rust like my favorite
/// macro assembler - C language ;)

fn main() {
    let res = prs_eplic::run();

    if let Err(e) = res {
        eprintln!("Error: {e:?}");
    }
}
