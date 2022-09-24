use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, Command};

type UniqrResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Cli {
    input_file: String,
    output_file: Option<String>,
    count: bool,
}

pub fn get_args() -> UniqrResult<Cli> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("wangxinyang")
        .about("Rust uniq")
        .arg(
            Arg::new("input_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::new("output_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("count")
                .short('c')
                .help("Show counts")
                .long("count")
                .takes_value(false),
        )
        .get_matches();

    Ok(Cli {
        input_file: matches.get_one::<String>("input_file").unwrap().to_string(),
        output_file: matches.get_one::<String>("output_file").map(Into::into),
        count: matches.is_present("count"),
    })
}

pub fn run(cli: Cli) -> UniqrResult<()> {
    match open(&cli.input_file) {
        Ok(mut file) => {
            let mut buf = String::new();
            let mut prev_key = String::new();
            let mut count = 0;
            loop {
                let bytes = file.read_line(&mut buf)?;
                if bytes == 0 {
                    break;
                }
                if prev_key.trim_end() != buf.trim_end() {
                    if count > 0 {
                        if cli.count {
                            print!("{:>8} {}", count, prev_key);
                        } else {
                            print!("{}", prev_key);
                        }
                    }
                    prev_key = buf.clone();
                    count = 0;
                }
                count += 1;
                buf.clear();
            }
            if count > 0 {
                if cli.count {
                    print!("{:>8} {}", count, prev_key);
                } else {
                    print!("{}", prev_key);
                }
            }
        }
        Err(e) => println!("{}: {}", cli.input_file, e),
    }

    Ok(())
}

pub fn open(filename: &str) -> UniqrResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
