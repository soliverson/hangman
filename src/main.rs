use rand::Rng;
use std::io;

fn main() {
    println!("Hello, Welcome to Christmas Hangman!");

    loop {
        // Word list for random selection
        let words = vec![
            "angel",
            "baptism",
            "blessing",
            "charity",
            "church",
            "covenant",
            "disciple",
            "eternity",
            "faith",
            "family",
            "forgiveness",
            "gospel",
            "grace",
            "heaven",
            "holy",
            "hope",
            "humility",
            "Jesus",
            "joy",
            "kindness",
            "love",
            "mercy",
            "miracle",
            "missionary",
            "ordinance",
            "patience",
            "prayer",
            "prophet",
            "redemption",
            "repentance",
            "sacrament",
            "savior",
            "scriptures",
            "service",
            "temple",
            "testimony",
            "tithing",
            "truth",
            "worship",
        ];
        let random_index = rand::thread_rng().gen_range(0..words.len()); // Generate random index
        let selected_word = words[random_index];

        let mut game = Hangman::new(selected_word, 10); // Updated max_attempts to 10
        let mut attempts = 0;

        while !game.is_won() && !game.is_lost(attempts) {
            println!("\nWord: {}", game.display_word());
            println!("Attempts left: {}", game.max_attempts - attempts);
            println!("Previous guesses: {:?}", game.guesses);

            // Get user input
            let mut input = String::new();
            println!("Enter your guess:");
            io::stdin().read_line(&mut input).unwrap();

            // Process input
            if let Some(guess) = input.trim().chars().next() {
                let guess = guess.to_lowercase().next().unwrap();
                if game.guess(guess) {
                    println!("Good guess!");
                } else {
                    println!("Wrong guess!");
                    attempts += 1;
                }
            } else {
                println!("Please enter a valid letter.");
            }

            println!();
        }

        if game.is_won() {
            println!("Congratulations! You guessed the word: {}", game.word);
        } else {
            println!("Game over! The word was: {}", game.word);
        }

        if !play_again() {
            println!("Thanks for playing Christmas Hangman! Have a Merry Christmas!");
            break;
        }
    }
}

struct Hangman {
    word: String,
    guesses: Vec<char>,
    max_attempts: u8,
}

impl Hangman {
    fn new(word: &str, max_attempts: u8) -> Hangman {
        Hangman {
            word: word.to_lowercase(),
            guesses: Vec::new(),
            max_attempts,
        }
    }

    fn guess(&mut self, letter: char) -> bool {
        if self.guesses.contains(&letter) || !letter.is_alphabetic() {
            return false;
        }

        self.guesses.push(letter);
        self.word.contains(letter)
    }

    fn display_word(&self) -> String {
        self.word
            .chars()
            .map(|c| if self.guesses.contains(&c) { c } else { '_' })
            .collect()
    }

    fn is_won(&self) -> bool {
        self.word.chars().all(|c| self.guesses.contains(&c))
    }

    fn is_lost(&self, attempts: u8) -> bool {
        attempts >= self.max_attempts
    }
}

fn play_again() -> bool {
    println!("Would you like to play again? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("y")
}
