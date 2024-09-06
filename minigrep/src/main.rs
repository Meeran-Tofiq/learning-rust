use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = parse_config(&args);

    println!("Searching for {}...", config.query);
    println!("Within {}", config.file_path);

    let content = fs::read_to_string(config.file_path)
    .expect("Couldn't read file data...");

    println!("With text content of:\n{}", content)
}

pub struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path}
}
