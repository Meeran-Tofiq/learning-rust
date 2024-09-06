use std::env;
use std::process;

use colored::Colorize;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{}", err.red());
        process::exit(1);
    });

    println!("Searching for {}...", config.query);
    println!("Within {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("{}", format!("Application error: {e}").red());
        process::exit(1);
    }
}
