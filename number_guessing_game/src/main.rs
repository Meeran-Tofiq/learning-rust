use std::io::{self, Write};
use rand::Rng;

fn main() {
    let random_number: i32 = rand::thread_rng().gen_range(1..=100); 
    println!("Guess numbers until you can guess the correct one!");
    
    print!("Your guess: ");
    io::stdout().flush().unwrap();

    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("Can only accept a number!");

    println!("You guessed {guess}")
}
