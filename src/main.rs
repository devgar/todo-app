#[derive(Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

impl Cli {
    pub fn new(pattern: & str, path: & str) -> Cli {
        Cli {
            pattern: String::from(pattern),
            path: std::path::PathBuf::from(path)
        }
    }
}

fn main() {

    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli::new(&pattern, &path);
    println!(" - pattern: {0:?}\n - path: {1:?}", args.pattern, args.path);
}
