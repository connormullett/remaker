use std::{collections::HashMap, error::Error, path::PathBuf};

use std::{env, fs, io, process};

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parse;
mod types;

const REMAKE_FILE_NAME: &str = "remaker";

fn find_remake_file() -> io::Result<PathBuf> {
    let mut current_dir = env::current_dir()?;
    current_dir.push(REMAKE_FILE_NAME);

    if let false = current_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "remake file not found",
        ));
    }

    Ok(current_dir)
}

fn error_and_die(error: Box<dyn Error>) {
    println!("{}", error);
    process::exit(1);
}

fn main() {
    let mut args = env::args().skip(1);

    let remake_file_path = match find_remake_file() {
        Ok(file) => file,
        Err(error) => {
            return error_and_die(Box::new(error));
        }
    };

    let remake_file_contents = match fs::read_to_string(remake_file_path) {
        Ok(content) => content,
        Err(error) => return error_and_die(Box::new(error)),
    };

    let remake_file = parse::parse(&remake_file_contents);

    let default_rule = match args.next() {
        Some(value) => value,
        None => remake_file.rules[0].target.to_string(),
    };

    println!("{}", default_rule);
    println!("file {:#?}", remake_file);

    let mut wildcards_map = HashMap::<String, String>::new();

    for wildcard in remake_file.wildcards {
        wildcards_map.insert(wildcard.symbol.to_string(), wildcard.values_as_string());
    }

    for rule in remake_file.rules {
        for mut command in rule.build_commands {}
    }

    println!("{:#?}", wildcards_map);
}
