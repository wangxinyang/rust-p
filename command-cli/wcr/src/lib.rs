use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{command, Arg};

type WcrResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Cli {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> WcrResult<Cli> {
    let mut matches = command!()
        .author("wangxinyang")
        .version("0.1.0")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple_values(true),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("Show word count")
                .takes_value(false),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Show byte count")
                .takes_value(false),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("Show character count")
                .takes_value(false)
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("Show line count")
                .takes_value(false),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|x| x == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Cli {
        files: matches.remove_many("files").unwrap().collect(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(cli: Cli) -> WcrResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    for filename in &cli.files {
        match open(filename) {
            Ok(file) => {
                if let Ok(file_info) = count(file) {
                    total_lines += file_info.lines;
                    total_words += file_info.words;
                    total_bytes += file_info.bytes;
                    total_chars += file_info.characters;
                    println!(
                        "{}{}{}{}{}",
                        format_field(file_info.lines, cli.lines),
                        format_field(file_info.words, cli.words),
                        format_field(file_info.bytes, cli.bytes),
                        format_field(file_info.characters, cli.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );
                }
            }
            Err(err) => eprintln!("{}: {}", filename, err),
        }
    }

    if cli.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, cli.lines),
            format_field(total_words, cli.words),
            format_field(total_bytes, cli.bytes),
            format_field(total_chars, cli.chars)
        );
    }
    Ok(())
}

pub fn open(filename: &str) -> WcrResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

pub struct FileInfo {
    lines: usize,
    words: usize,
    bytes: usize,
    characters: usize,
}

fn count(mut file: Box<dyn BufRead>) -> WcrResult<FileInfo> {
    let mut buf = String::new();
    let mut line_num = 0;
    let mut words_num = 0;
    let mut bytes_num = 0;
    let mut char_num = 0;
    loop {
        let bytes = file.read_line(&mut buf)?;
        if bytes == 0 {
            break;
        }
        bytes_num += bytes;
        line_num += 1;
        words_num += buf.split_whitespace().count();
        char_num += buf.chars().count();

        buf.clear();
    }

    Ok(FileInfo {
        lines: line_num,
        words: words_num,
        bytes: bytes_num,
        characters: char_num,
    })
}
