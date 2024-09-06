use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    println!("Searching for {}...", config.query);
    println!("Within {}", config.file_path);

    run(config)
}

fn run(config: Config) {
    let content = fs::read_to_string(config.file_path)
    .expect("Couldn't read file data...");

    println!("With text content of:\n{content}");
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: <command> <query> <file_path>");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Self { query, file_path})
    }
}
