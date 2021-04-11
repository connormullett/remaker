use std::{error::Error, path::PathBuf};

use std::{env, fs, io, process};

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "remake.pest"]
pub struct RemakeParser;

#[derive(Debug, Clone)]
struct RemakeRule<'a> {
    targets: Vec<&'a str>,
    dependencies: Vec<&'a str>,
    build_commands: Vec<&'a str>,
}

impl<'a> RemakeRule<'a> {
    pub fn new() -> Self {
        Self {
            targets: vec![],
            dependencies: vec![],
            build_commands: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.targets = vec![];
        self.dependencies = vec![];
        self.build_commands = vec![];
    }
}

#[derive(Debug, Clone)]
struct RemakeWildcard<'a> {
    symbol: &'a str,
    values: Vec<&'a str>,
}

impl<'a> RemakeWildcard<'a> {
    pub fn new() -> Self {
        Self {
            symbol: "",
            values: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.symbol = "";
        self.values = vec![];
    }
}

#[derive(Debug)]
struct RemakeFile<'a> {
    rules: Vec<RemakeRule<'a>>,
    wildcards: Vec<RemakeWildcard<'a>>,
}

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

    let file = RemakeParser::parse(Rule::remake_file, &&remake_file_contents)
        .expect("bad parse")
        .next()
        .unwrap();

    let mut wildcards = Vec::<RemakeWildcard>::new();

    let mut rules = Vec::<RemakeRule>::new();
    let mut current_rule = RemakeRule::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::wildcard => {
                let mut inner_rules = line.into_inner();
                let symbol: &str = inner_rules.next().unwrap().as_str();
                let mut current_wildcard = RemakeWildcard::new();
                current_wildcard.symbol = symbol;

                while let Some(value) = inner_rules.next() {
                    current_wildcard.values.push(value.as_str());
                }
                wildcards.push(current_wildcard.clone());
                current_wildcard.clear();
            }
            Rule::target_line => {
                rules.push(current_rule.clone());
                current_rule.clear();
                let mut inner_rules = line.into_inner();
                let target = inner_rules.next().unwrap().as_str();
                let dependencies = inner_rules.next().unwrap().as_str();
                current_rule = RemakeRule {
                    targets: vec![target],
                    dependencies: vec![dependencies],
                    build_commands: vec![],
                };
            }
            Rule::build_command => {
                let rules = line.into_inner();
                current_rule.build_commands.push(rules.as_str())
            }
            Rule::EOI => (),
            _ => (),
        }
    }

    let remake_file = RemakeFile {
        rules,
        wildcards: wildcards.clone(),
    };

    println!("file {:#?}", remake_file);
}
