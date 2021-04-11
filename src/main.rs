use std::{error::Error, path::PathBuf};

use std::{env, fs, io, process};

use types::RemakeRule;

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

    let mut remake_file = parse::parse(&remake_file_contents);

    remake_file.handle_wildcards();

    let default_rule_name = match args.next() {
        Some(value) => value,
        None => remake_file.rules[0].target.to_string(),
    };

    let mut default_rule: Option<RemakeRule> = None;
    for rule in remake_file.rules.iter() {
        if rule.target == default_rule_name {
            default_rule = Some(rule.clone());
        }
    }

    let target_metadata = fs::metadata(default_rule.unwrap().target_as_path());
    match target_metadata {
        Ok(value) => println!("modified {:?}", value.modified()),
        Err(error) => error_and_die(Box::new(error)),
    }
}
