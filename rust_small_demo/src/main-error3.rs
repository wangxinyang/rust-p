use std::{
    error::Error,
    fs::File,
    io::{self, Read},
    num::ParseIntError,
    path::Path,
};

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(ParseIntError),
}

fn main() {
    match file_double("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}

// impl From<io::Error> for CliError {
//     fn from(err: io::Error) -> Self {
//         CliError::Io(err)
//     }
// }

// impl From<ParseIntError> for CliError {
//     fn from(err: ParseIntError) -> Self {
//         CliError::Parse(err)
//     }
// }

fn file_double<P: AsRef<Path>>(path: P) -> Result<i32, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n: i32 = contents.trim().parse()?;
    Ok(2 * n)
}
