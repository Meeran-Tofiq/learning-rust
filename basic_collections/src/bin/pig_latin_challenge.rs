use std::io;

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];
const SUFFIXES: [&str; 2] = ["ay", "hay"];

enum FirstLetterState {
    Vowel,
    Consonant
}

fn check_first_letter(word: &str) -> FirstLetterState {
    let first_letter = &word[0..1];

    if VOWELS.contains(&first_letter) {
        return FirstLetterState::Vowel;
    }

    FirstLetterState::Consonant
}

fn main(){
    let mut word: String = String::new();
    io::stdin().read_line(&mut word).expect("Could not read your word");
}