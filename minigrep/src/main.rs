use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::new(&args);

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

impl Config {
    fn new(args: &Vec<String>) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Self { query, file_path}
    }
}
