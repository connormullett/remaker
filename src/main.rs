use std::path::PathBuf;

use std::env;
use std::io;
use std::process;

const REMAKE_FILE_NAME: &'static str = "remaker";

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

fn main() {
    let remake_file = match find_remake_file() {
        Ok(file) => file,
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    };

    println!("remake_file {:?}", remake_file);
}
