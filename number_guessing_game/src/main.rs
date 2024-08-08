use std::io::{self, Write};

fn main() {
    println!("Guess numbers until you can guess the correct one!");
    print!("Your guess: ");
    io::stdout().flush().unwrap();

    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess).expect("Can only accept a number!");

    println!("You guessed {guess}")
}
