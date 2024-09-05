use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let [_, query, file_path] = &args[..] else {
        eprintln!("Usage: <program> <query> <file>");
        return;
    };

    println!("Searching for {}...", query);
    println!("Within {}", file_path);

    let content = fs::read_to_string(file_path)
    .expect("Couldn't read file data...");

    println!("With text content of:\n{}", content)
}
