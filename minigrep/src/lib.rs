use std::io::prelude::*;
use std::{error::Error, fs::File};

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let mut f: File = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query: String = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string"),
        };
        let filename: String = match args.next() {
            Some(filename) => filename,
            None => return Err("Didn't get a file name"),
        };

        Ok(Self { query, filename})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod test;
