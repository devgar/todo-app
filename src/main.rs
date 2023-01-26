use std::env::args;
use std::path::PathBuf;

use regex::Regex;

#[derive(Debug)]
struct Cli { pattern: Regex, path: PathBuf }

impl Cli {
    pub fn new(pattern: & str, path: & str) -> Cli {
        Cli {
            pattern: Regex::new(pattern).expect("Invalid regex"),
            path: PathBuf::from(path)
        }
    }
}

fn main() {

    let pattern = args().nth(1).expect("no pattern given");
    let path = args().nth(2).expect("no path given");
    println!(" - pattern: {0:?}\n - path: {1:?}", &pattern, &path);
    let args = Cli::new(&pattern, &path);
    println!("Args built: {:?}", args);
}
