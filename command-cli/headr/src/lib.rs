use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author = "tosei", version = "0.1.0", about = "Rust head")]
pub struct Args {
    #[clap()]
    files: Vec<String>,

    /// Number of lines [default: 10]
    #[clap(
        short = 'n',
        long = "--lines",
        conflicts_with = "BYTES",
        required = false,
        takes_value = true,
        default_value = "10"
    )]
    lines: String,

    /// Number of bytes
    #[clap(
        name = "BYTES",
        short = 'c',
        long = "--bytes",
        required = false,
        takes_value = true
    )]
    bytes: Option<String>,
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();

    let lines = match parse_positive_int(&args.lines) {
        Some(number) => number,
        None => return Err(From::from(format!("illegal line count -- {}", args.lines))),
    };

    let bytes: Option<String> = match args.bytes {
        Some(bytes) => match parse_positive_int(&bytes) {
            Some(bytes) => Some(bytes.to_string()),
            None => return Err(From::from(format!("illegal byte count -- {}", &bytes))),
        },
        _ => None,
    };

    Ok(Args {
        files: if args.files.is_empty() {
            vec!["-".to_string()]
        } else {
            args.files
        },
        lines: lines.to_string(),
        bytes,
    })
}

pub fn run(args: Args) -> MyResult<()> {
    let num_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(ref num_bytes) = args.bytes {
                    let bytes: usize = num_bytes.parse()?;
                    let mut handle = file.take(bytes as u64);
                    let mut buffer = vec![0; bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines.parse()? {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> Option<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Some(n),
        _ => None,
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
