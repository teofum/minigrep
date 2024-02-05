use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        eprintln!("Unable to parse arguments: {err}");
        eprintln!("Usage: minigrep <query> <file_path>");
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Error: {err}");
        process::exit(1);
    };
}
