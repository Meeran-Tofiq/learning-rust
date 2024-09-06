use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let (query, file_path): (&str, &str) = parse_config(&args);

    println!("Searching for {}...", query);
    println!("Within {}", file_path);

    let content = fs::read_to_string(file_path)
    .expect("Couldn't read file data...");

    println!("With text content of:\n{}", content)
}

fn parse_config(args: &Vec<String>) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
