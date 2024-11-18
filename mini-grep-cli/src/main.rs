use std::env;
use std::process;

use mini_grep_cli::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = mini_grep_cli::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
