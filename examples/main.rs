use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Hangman!");

    let word = "Christmas";
    let max_attempts = 6;

    let mut hangman = Hangman::new(word, max_attempts);

    while !hangman.is_won() && !hangman.is_lost() {
        println!("Word: {}", hangman.display_word());
        println!("Attempts left: {}", hangman.attempts_left());
        println!("Guesses: {:?}", hangman.get_guesses());

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let guess = input.trim();

        if guess.len() != 1 {
            println!("Please enter only a single letter.");
            continue;
        }

        if hangman.guess(guess.to_string()) {
            println!("Good guess!");
        } else {
            println!("Wrong guess!");
        }
        println!();
    }

    if hangman.is_won() {
        println!("Congratulations! You guessed the word: {}", word);
    } else {
        println!("Game over! The word was: {}", word);
    }
}

pub struct Hangman {
    word: String,
    guesses: HashSet<String>,
    max_attempts: u8,
}

impl Hangman {
    pub fn new(word: &str, max_attempts: u8) -> Hangman {
        Hangman {
            word: word.to_lowercase(),
            guesses: HashSet::new(),
            max_attempts,
        }
    }

    pub fn guess(&mut self, letter: String) -> bool {
        let letter = letter.to_lowercase();
        self.guesses.insert(letter.clone());
        self.word.contains(&letter)
    }

    pub fn display_word(&self) -> String {
        self.word
            .chars()
            .map(|c| {
                if self.guesses.contains(&c.to_string()) {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }

    pub fn attempts_left(&self) -> u8 {
        let wrong_guesses = self
            .guesses
            .iter()
            .filter(|&guess| !self.word.contains(guess))
            .count() as u8;
        self.max_attempts.saturating_sub(wrong_guesses)
    }

    pub fn is_won(&self) -> bool {
        self.word
            .chars()
            .all(|c| self.guesses.contains(&c.to_string()))
    }

    pub fn is_lost(&self) -> bool {
        self.attempts_left() == 0
    }

    pub fn get_guesses(&self) -> Vec<String> {
        self.guesses.iter().cloned().collect()
    }
}
