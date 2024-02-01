use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        println!("Unable to parse arguments: {err}");
        println!("Usage: minigrep <query> <file_path>");
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path).expect("Should be able to read the file");

    println!("File contents:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn from_args(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
