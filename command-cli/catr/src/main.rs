use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser)]
#[clap(author = "tosei", version = "0.1.0", about = "Rust cat")]
struct Args {
    files: Vec<String>,

    /// Number the output lines, starting at 1.
    #[clap(
        short = 'n',
        long = "number",
        conflicts_with = "number_nonblank",
        help = "Number lines"
    )]
    number_lines: bool,

    /// Number the non-blank output lines, starting at 1.
    #[clap(
        name = "number_nonblank",
        short = 'b',
        long = "number-nonblank",
        help = "Number non-blank lines"
    )]
    number_nonblank_lines: bool,
}

fn main() -> MyResult<()> {
    let args = Args::parse();
    let number_lines = args.number_lines;
    let number_nonblank_lines = args.number_nonblank_lines;
    let mut files = args.files;
    if files.is_empty() {
        files.push("-".to_string());
    }
    for file in files {
        match open(&file) {
            Err(err) => eprintln!("Failed to open {}: {}", file, err),
            Ok(file) => {
                let mut last_line = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if number_nonblank_lines {
                        if !line.is_empty() {
                            last_line += 1;
                            println!("{:>6}\t{}", last_line, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }

    Ok(())
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
