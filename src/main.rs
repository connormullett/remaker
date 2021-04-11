use std::{error::Error, path::PathBuf, time::SystemTime};

use std::{env, fs, io, process};

use types::{RemakeFile, RemakeRule};

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

fn get_modified_time_from_path(path: &PathBuf) -> SystemTime {
    match fs::metadata(path) {
        Ok(value) => value.modified().unwrap(),
        Err(_) => SystemTime::UNIX_EPOCH,
    }
}

fn process_rules(default_rule_name: String, remake_file: RemakeFile) {
    let mut default_rule: Option<RemakeRule> = None;
    for rule in remake_file.rules.iter() {
        if rule.target == default_rule_name {
            default_rule = Some(rule.clone());
        }
    }
    let rule = default_rule.unwrap();

    process_rule(&rule, &remake_file);
}

fn process_rule(rule: &RemakeRule, remake_file: &RemakeFile) {
    let target_modified = get_modified_time_from_path(&rule.target_as_path());
    let current_dir = env::current_dir().unwrap();

    for dependency in &rule.dependencies {
        let mut dependency_path = current_dir.clone();
        dependency_path.push(PathBuf::from(&dependency));
        let dependency_modified = get_modified_time_from_path(&dependency_path);

        if target_modified >= dependency_modified {
            for dep_rule in &remake_file.rules {
                if dep_rule.target.eq(dependency) {
                    process_rule(&dep_rule, remake_file);
                }
            }
        }
        rule.run_build_commands();
    }
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

    process_rules(default_rule_name, remake_file);
}
