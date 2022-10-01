use anyhow::Result;
use clap::{command, Parser, ValueEnum};
use std::ops::Range;

type PositionList = Vec<Range<usize>>;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = "Rust cut")]
#[command(next_line_help = true)]
pub struct CutCommand {
    #[arg(value_name = "FILES", default_value = "-")]
    files: Vec<String>,

    #[arg(short, long, default_value = "\t")]
    delimiter: u8,
    // #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..))]
    // extract: Extract,
}

// #[derive(Debug, Clone)]
// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// enum Extract {
//     Fields(PositionList),
//     Bytes(PositionList),
//     Chars(PositionList),
// }

// pub fn get_args() -> Result<CutCommand> {
//     let matches = Command::new("cutr")
//         .author("wxyang.tk")
//         .about("Rust cut")
//         .version("0.1.0")
//         .arg(
//             Arg::new("file")
//                 .value_name("FILE")
//                 .help("Input file(s) [default: -]")
//                 .multiple_values(true)
//                 .default_value("-"),
//         )
//         .arg(Arg::new("delimiter").value_name("DELIMITER").short('d'))
//         .get_matches();

//     Ok(Cli {
//         files: todo!(),
//         delimiter: todo!(),
//         extract: todo!(),
//     })
// }

pub fn run(cli: CutCommand) -> Result<()> {
    println!("{:?}", cli);
    Ok(())
}
