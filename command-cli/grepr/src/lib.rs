use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    mem::take,
};

use anyhow::{Ok, Result};
use clap::{command, Parser};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
// #[command(next_line_help = true)]
pub struct GrepCli {
    /// PATTERN
    parttern: Regex,

    /// FILE
    #[clap(default_value = "-")]
    file: Vec<String>,

    /// Recursive search
    #[clap(short, long, default_value = "false")]
    recursive: bool,

    /// Count occurrences
    #[clap(short, long, default_value = "false")]
    count: bool,

    /// Invert match
    #[clap(short = 'v', long, default_value = "false")]
    invert_match: bool,

    /// Case-insensitive
    #[clap(short, long, default_value = "false")]
    insensitive: bool,
}

pub fn run() -> Result<()> {
    let grep_cli = GrepCli::parse();

    let result = find_files(grep_cli.file, grep_cli.recursive)?;

    for file in result.iter() {
        match open(file) {
            core::result::Result::Ok(buf_read) => {
                match find_lines(buf_read, &grep_cli.parttern, grep_cli.invert_match) {
                    Err(e) => eprintln!("{}", e),
                    core::result::Result::Ok(matches) => {
                        if grep_cli.count {
                            print!("{}:{}", &file, &format!("{}\n", matches.len()));
                        } else {
                            for line in matches {
                                print!("{}:{}", &file, line);
                            }
                        }
                    }
                }
            }
            Err(err) => eprintln!("Failed to open {}: {}", file, err),
        }
    }

    Ok(())
}

fn find_lines<T: BufRead>(
    mut buf_read: T,
    pattern: &Regex,
    invert_match: bool,
) -> Result<Vec<String>> {
    let mut matches = Vec::new();
    let mut line = String::new();
    loop {
        let size = buf_read.read_line(&mut line)?;
        if size == 0 {
            break;
        }
        if pattern.is_match(&line) ^ invert_match {
            matches.push(take(&mut line));
        }
        line.clear();
    }

    Ok(matches)
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn find_files(pathes: Vec<String>, is_recursive: bool) -> Result<Vec<String>> {
    let mut result = Vec::new();
    for path in pathes {
        match path.as_str() {
            "-" => result.push(path.to_string()),
            _ => match fs::metadata(path.as_str()) {
                core::result::Result::Ok(metadata) => {
                    if metadata.is_dir() {
                        if is_recursive {
                            for entry in WalkDir::new(path) {
                                match entry {
                                    Err(e) => eprintln!("{}", e),
                                    core::result::Result::Ok(entry) => {
                                        result.push(entry.path().display().to_string())
                                    }
                                }
                            }
                        } else {
                            eprintln!("{} is a directory", path);
                        }
                    } else if metadata.is_file() {
                        result.push(path)
                    }
                }
                Err(_) => eprintln!("test"),
            },
        }
    }
    Ok(result)
}
