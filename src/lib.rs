use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, KeyboardEvent};

#[wasm_bindgen]
pub struct Hangman {
    word: String,
    guesses: HashSet<String>,
    max_attempts: u8,
}

#[wasm_bindgen]
impl Hangman {
    #[wasm_bindgen(constructor)]
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
                if c == ' ' {
                    " ".to_string() // Show space directly
                } else if self.guesses.contains(&c.to_string()) {
                    c.to_string()
                } else {
                    "_".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ") // Add a single space between letters
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
            .filter(|&c| c != ' ') // Ignore spaces when checking for win
            .all(|c| self.guesses.contains(&c.to_string()))
    }

    pub fn is_lost(&self) -> bool {
        self.attempts_left() == 0
    }

    pub fn get_guesses(&self) -> String {
        self.guesses.iter().cloned().collect::<Vec<_>>().join(", ")
    }

    pub fn get_word(&self) -> String {
        self.word.clone()
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let words = vec![
        "Jesus",
        "Silent Night",
        "Manger",
        "Star of Bethlehem",
        "Candy Cane",
        "Reindeer",
        "Santa Claus",
        "Christmas Tree",
        "Holly Jolly",
        "Frosty the Snowman",
    ];

    let document = web_sys::window()
        .and_then(|win| win.document())
        .ok_or_else(|| JsValue::from_str("Could not access document"))?;

    let word_display = Rc::new(document.get_element_by_id("word-display").unwrap());
    let attempts_display = Rc::new(document.get_element_by_id("attempts").unwrap());
    let guesses_display = Rc::new(document.get_element_by_id("guesses").unwrap());
    let input_element = Rc::new(
        document
            .get_element_by_id("guess-input")
            .unwrap()
            .dyn_into::<HtmlInputElement>()?,
    );
    let status_display = Rc::new(document.get_element_by_id("status").unwrap());
    let guess_button = Rc::new(document.get_element_by_id("guess-button").unwrap());
    let restart_button = Rc::new(document.get_element_by_id("restart-button").unwrap());

    let game = Rc::new(RefCell::new(Hangman::new(
        &words[rand::random::<usize>() % words.len()],
        10,
    )));

    let update_game_display = Rc::new({
        let game = game.clone();
        let word_display = word_display.clone();
        let attempts_display = attempts_display.clone();
        let guesses_display = guesses_display.clone();

        move || {
            let game = game.borrow();
            word_display.set_text_content(Some(&game.display_word()));
            attempts_display
                .set_text_content(Some(&format!("Attempts Left: {}", game.attempts_left())));
            guesses_display.set_text_content(Some(&format!(
                "Guessed Letters: {}",
                if game.get_guesses().is_empty() {
                    "None".to_string()
                } else {
                    game.get_guesses()
                }
            )));
        }
    });

    let handle_guess = Rc::new({
        let game = game.clone();
        let update_game_display = update_game_display.clone();
        let status_display = status_display.clone();
        let input_element = input_element.clone();

        move || {
            let letter = input_element.value();
            input_element.set_value("");

            if letter.len() != 1 {
                status_display.set_text_content(Some("Enter a single letter!"));
                return;
            }
            if letter.chars().any(|c| c.is_numeric()) {
                status_display.set_text_content(Some("Numbers are not allowed!"));
                return;
            }

            if game.borrow_mut().guess(letter.clone()) {
                status_display.set_text_content(Some("Good guess!"));
            } else {
                status_display.set_text_content(Some("Wrong guess!"));
            }

            update_game_display();

            let game_ref = game.borrow();
            if game_ref.is_won() {
                status_display.set_text_content(Some("You won!"));
            } else if game_ref.is_lost() {
                status_display.set_text_content(Some(&format!(
                    "Game over! The word was {}.",
                    game_ref.get_word()
                )));
            }
        }
    });

    let keydown_closure = Closure::wrap(Box::new({
        let handle_guess = handle_guess.clone();
        move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                handle_guess();
            }
        }
    }) as Box<dyn FnMut(_)>);
    input_element
        .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())?;
    keydown_closure.forget();

    let guess_button_closure = Closure::wrap(Box::new({
        let handle_guess = handle_guess.clone();
        move || {
            handle_guess();
        }
    }) as Box<dyn FnMut()>);
    guess_button
        .add_event_listener_with_callback("click", guess_button_closure.as_ref().unchecked_ref())?;
    guess_button_closure.forget();

    let restart_closure = Closure::wrap(Box::new({
        let game = game.clone();
        let update_game_display = update_game_display.clone();
        let status_display = status_display.clone();
        move || {
            *game.borrow_mut() = Hangman::new(&words[rand::random::<usize>() % words.len()], 10);
            update_game_display();
            status_display.set_text_content(Some("Game restarted! Start guessing."));
        }
    }) as Box<dyn FnMut()>);
    restart_button
        .add_event_listener_with_callback("click", restart_closure.as_ref().unchecked_ref())?;
    restart_closure.forget();

    update_game_display();
    status_display.set_text_content(Some("Game started! Start guessing."));

    Ok(())
}
