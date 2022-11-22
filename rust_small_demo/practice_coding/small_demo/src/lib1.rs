use std::{error::Error, fs};

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            vec.push(line)
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust: 
safe, fast, productive.
shhsh ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
