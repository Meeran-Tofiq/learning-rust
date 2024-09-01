use std::fs::File;
use std::io::{self, Read};

fn main() {
    let meh = read_username_from_file();
    println!("{meh:?}");
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
