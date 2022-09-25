use std::error::Error;

use clap::{parser::ValuesRef, Arg, Command};
use regex::Regex;
use walkdir::WalkDir;

type FindResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum FileType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Cli {
    files: Vec<String>,
    ftypes: Vec<FileType>,
    names: Vec<Regex>,
}

pub fn get_args() -> FindResult<Cli> {
    let mut matches = Command::new("")
        .version("0.1.0")
        .author("wxyang")
        .about("Rust find")
        .arg(
            Arg::new("files")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .multiple_values(true),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n')
                .long("name")
                .help("Name")
                .takes_value(true),
        )
        .arg(
            Arg::new("ftypes")
                .value_name("TYPES")
                .short('t')
                .long("type")
                .help("Entry type [possible values: f, d, l]")
                .takes_value(true),
        )
        .get_matches();

    let names = matches
        .get_many("names")
        .map(|vals: ValuesRef<String>| {
            vals.map(|name| Regex::new(name).map_err(|_| format!("Invalid --name \"{}\"", name)))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .get_many("ftypes")
        .map(|ftypes: ValuesRef<String>| {
            ftypes
                .map(|ftype| match ftype.as_str() {
                    "d" => FileType::Dir,
                    "f" => FileType::File,
                    "l" => FileType::Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect::<Vec<FileType>>()
        })
        .unwrap_or_default();

    Ok(Cli {
        files: matches.remove_many::<String>("files").unwrap().collect(),
        ftypes: entry_types,
        names,
    })
}

pub fn run(cli: Cli) -> FindResult<()> {
    for path in cli.files {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }
    Ok(())
}
