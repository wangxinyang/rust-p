use anyhow::Result;
use clap::{Arg, Command};
use std::ops::Range;

type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub struct Cli {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

#[derive(Debug)]
enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

pub fn get_args() -> Result<Cli> {
    let matches = Command::new("cutr")
        .author("wxyang.tk")
        .about("Rust cut")
        .version("0.1.0")
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Input file(s) [default: -]")
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(Arg::new("delimiter").value_name("DELIMITER").short('d'))
        .get_matches();

    Ok(Cli {
        files: todo!(),
        delimiter: todo!(),
        extract: todo!(),
    })
}

pub fn run(cli: Cli) -> Result<()> {
    println!("{:?}", cli);
    Ok(())
}
