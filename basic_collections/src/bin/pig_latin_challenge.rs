use std::io::{self, Write};

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];
const SUFFIXES: [&str; 2] = ["ay", "hay"];

enum FirstLetterState {
    Vowel,
    Consonant
}

fn check_first_letter(word: &str) -> FirstLetterState {
    let first_letter = &word[0..1].to_lowercase();

    if VOWELS.contains(&first_letter.as_str()) {
        return FirstLetterState::Vowel;
    }

    FirstLetterState::Consonant
}

fn transform_consonanted_word(word: &str) -> String {
    let first_letter = &word[0..1];
    let word_remainder = &word[1..];
    return format!("{}-{}{}", word_remainder, first_letter, SUFFIXES[0]);
}

fn transform_voweled_word(word: &str) -> String {
    return format!("{}-{}", word, SUFFIXES[1]);
}

fn main(){
    print!("Your word to be transformed into pig latin: ");
    io::stdout().flush().expect("How did this error even happen?");

    let mut word: String = String::new();
    io::stdin().read_line(&mut word).expect("Could not read your word");
    word = word.trim().to_string();

    let result: String = match check_first_letter(&word) {
        FirstLetterState::Consonant => transform_consonanted_word(&word),
        FirstLetterState::Vowel =>  transform_voweled_word(&word)
    };

    println!("{word} in pig latin is {result}");
}