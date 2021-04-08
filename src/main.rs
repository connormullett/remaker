use std::{
    error::Error,
    path::{Path, PathBuf},
};

use std::{env, fs, io, process};

const REMAKE_FILE_NAME: &str = "remaker";

#[derive(Debug)]
struct Rule {
    targets: Vec<Box<Path>>,
    dependencies: Vec<Box<Path>>,
    build_steps: Vec<String>,
}

impl Rule {
    pub fn from(
        targets: Vec<Box<Path>>,
        dependencies: Vec<Box<Path>>,
        build_steps: Vec<String>,
    ) -> Self {
        Self {
            targets,
            dependencies,
            build_steps,
        }
    }
}

type Rules = Vec<Rule>;

fn parse_rule(_input: String) -> Rule {
    Rule::from(vec![], vec![], vec![])
}

fn parse_remake_file(_remake_file_contents: String) -> Rules {
    parse_rule(String::new());
    Rules::new()
}

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

fn read_remake_file(path: PathBuf) -> io::Result<String> {
    let buffer = fs::read_to_string(path)?;
    Ok(buffer)
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

    let remake_file_contents = match read_remake_file(remake_file_path) {
        Ok(content) => content,
        Err(error) => return error_and_die(Box::new(error)),
    };

    let rules = parse_remake_file(remake_file_contents);

    println!("{:?}", rules);
}
