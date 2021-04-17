use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

use std::{env, fs, io, process};

use clap::{App, Arg};
use types::{RemakeFile, RemakeRule};

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parse;
mod types;

const REMAKE_FILE_NAME: &str = "remaker";

fn find_remake_file(file_name: Option<&str>) -> io::Result<PathBuf> {
    let mut current_dir = env::current_dir()?;
    match file_name {
        Some(value) => current_dir.push(value),
        None => current_dir.push(REMAKE_FILE_NAME),
    };

    if let false = current_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "remake file not found",
        ));
    }

    Ok(current_dir)
}

fn error_and_die(error_msg: String) {
    eprintln!("{}", error_msg);
    process::exit(1);
}

fn get_modified_time_from_path(path: &Path) -> SystemTime {
    match fs::metadata(path) {
        Ok(value) => value.modified().unwrap(),
        Err(_) => SystemTime::UNIX_EPOCH,
    }
}

fn create_full_path_from_string(name: String) -> PathBuf {
    let mut out = env::current_dir().unwrap();
    out.push(PathBuf::from(name));
    out
}

fn process_rules(default_rule_name: String, remake_file: RemakeFile) {
    let mut default_rule: Option<RemakeRule> = None;
    for rule in remake_file.rules.iter() {
        if rule.target == default_rule_name {
            default_rule = Some(rule.clone());
            break;
        }
    }

    let rule = match default_rule {
        Some(value) => value,
        None => return error_and_die(format!("No rule by name '{}'", default_rule_name)),
    };

    if rule.dependencies.is_empty() {
        rule.run_build_commands();
    } else {
        process_rule(&rule, &remake_file);
        rule.run_build_commands();
    }
}

fn process_rule(rule: &RemakeRule, remake_file: &RemakeFile) {
    let target_path = create_full_path_from_string(rule.target.clone());
    let target_modified = get_modified_time_from_path(&target_path);

    for dependency in &rule.dependencies {
        let dependency_path = create_full_path_from_string(dependency.clone());
        let dependency_modified = get_modified_time_from_path(&dependency_path);

        if target_modified >= dependency_modified {
            for dep_rule in &remake_file.rules {
                if dep_rule.target.eq(dependency) {
                    process_rule(&dep_rule, remake_file);
                }
            }
        } else {
            rule.run_build_commands();
        }
    }
}

fn main() {
    let matches = App::new("Remake")
        .author("github.com/connormullett")
        .about("A GNU make clone written in rust")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("specify remake file location")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("RULE")
                .help("specify an optional default rule")
                .required(false)
                .index(1),
        )
        .get_matches();

    let defined_remake_file = matches.value_of("path");

    let remake_file_path = match find_remake_file(defined_remake_file) {
        Ok(file) => file,
        Err(_) => {
            return error_and_die(format!(
                "Can't find remake file '{}'",
                defined_remake_file.unwrap_or(REMAKE_FILE_NAME)
            ));
        }
    };

    let remake_file_contents = match fs::read_to_string(&remake_file_path) {
        Ok(content) => content,
        Err(_) => {
            return error_and_die(format!(
                "error reading file '{}'",
                remake_file_path.to_string_lossy()
            ))
        }
    };

    let mut remake_file = parse::parse(&remake_file_contents);

    remake_file.handle_wildcards();

    let default_rule = match matches.value_of("RULE") {
        Some(value) => value.to_string(),
        None => remake_file.rules[0].target.to_string(),
    };

    process_rules(default_rule, remake_file);
}
