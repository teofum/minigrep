use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = match args.get(1) {
        Some(arg) => arg,
        None => {
            println!("Must specify query. Usage: minigrep <query> <file_path>");
            return;
        }
    };

    let file_path = match args.get(2) {
        Some(arg) => arg,
        None => {
            println!("Must specify file path. Usage: minigrep <query> <file_path>");
            return;
        }
    };

    println!("Searching for {query} in {file_path}");
}
