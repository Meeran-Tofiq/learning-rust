use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let random_number: i32 = rand::thread_rng().gen_range(1..=100); 
    println!("Guess numbers until you can guess the correct one!");
    
    loop {
        print!("Your guess: ");
        io::stdout().flush().expect("How did this error even happen?");

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Could not read your line!");
        let guess: i32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(err) => {
                println!("{:?}", err.to_string());
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("The number will always be between 1 and 100.");
            continue;
        }

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Your number was less than the secret number."),
            Ordering::Greater => println!("Your number was greater than the secret number"),
            Ordering::Equal => {
                println!("Spot on!");
                break;
            }
        }
        
    }
}
