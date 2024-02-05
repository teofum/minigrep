use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        println!("Unable to parse arguments: {err}");
        println!("Usage: minigrep <query> <file_path>");
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);

    if let Err(err) = minigrep::run(config) {
        println!("Error: {err}");
        process::exit(1);
    };
}
