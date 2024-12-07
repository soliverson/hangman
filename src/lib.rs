use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, KeyboardEvent};

#[wasm_bindgen]
pub struct Hangman {
    word: String,
    guesses: HashSet<String>,
    max_attempts: u8,
    wins: u32,
    losses: u32,
    game_over: bool,
}

#[wasm_bindgen]
impl Hangman {
    #[wasm_bindgen(constructor)]
    pub fn new(word: &str, max_attempts: u8) -> Hangman {
        Hangman {
            word: word.to_lowercase(),
            guesses: HashSet::new(),
            max_attempts,
            wins: 0,
            losses: 0,
            game_over: false,
        }
    }

    pub fn guess(&mut self, letter: String) -> bool {
        if self.game_over {
            // No guesses allowed after game ends
            return false;
        }

        let letter = letter.to_lowercase();
        self.guesses.insert(letter.clone());
        self.word.contains(&letter)
    }

    pub fn display_word(&self) -> String {
        self.word
            .chars()
            .map(|c| {
                if c == ' ' {
                    "<span class='space'></span>".to_string()
                } else if self.guesses.contains(&c.to_string()) {
                    format!("<span>{}</span>", c)
                } else {
                    "<span>_</span>".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn attempts_left(&self) -> u8 {
        let wrong_guesses = self
            .guesses
            .iter()
            .filter(|&guess| !self.word.contains(guess))
            .count() as u8;
        self.max_attempts.saturating_sub(wrong_guesses)
    }

    pub fn is_won(&mut self) -> bool {
        if !self.game_over
            && self
                .word
                .chars()
                .filter(|&c| c != ' ')
                .all(|c| self.guesses.contains(&c.to_string()))
        {
            self.wins += 1;
            self.game_over = true;
            true
        } else {
            false
        }
    }

    pub fn is_lost(&mut self) -> bool {
        if !self.game_over && self.attempts_left() == 0 {
            self.losses += 1;
            self.game_over = true;
            true
        } else {
            false
        }
    }

    pub fn get_guesses(&self) -> String {
        self.guesses.iter().cloned().collect::<Vec<_>>().join(", ")
    }

    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn get_score(&self) -> String {
        format!("Wins: {}, Losses: {}", self.wins, self.losses)
    }

    pub fn reset_game(&mut self, new_word: &str) {
        self.word = new_word.to_lowercase();
        self.guesses.clear();
        self.game_over = false;
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let words = vec![
        "Merry Christmas",
        "Baby Jesus",
        "Mary and Joseph",
        "A Christmas Carol",
        "Advent calendar",
        "Angels from heaven",
        "Angels joy",
        "Angels rejoice",
        "Angels sing",
        "Away in a manger",
        "Bethlehem star",
        "Jingle all the way",
        "Candlelight service",
        "Candy canes",
        "Caroling nights",
        "The polar express",
        "Charlie Brown Christmas",
        "Chestnuts roasting",
        "Frosty the snowman",
        "Christ the Savior",
        "Jack Frost",
        "Christmas",
        "White Christmas",
        "Ghost of Christmas present",
        "Ghost of Christmas future",
        "Ghost of Christmas past",
        "Christmas Eve magic",
        "Decorate the tree",
        "Christmas Eve songs",
        "Holiday",
        "Snowman",
        "Reindeer",
        "Santa",
        "Elves",
        "Chimney",
        "Mistletoe",
        "Tinsel",
        "Twinkling lights",
        "Christmas lights",
        "Ornament",
        "Wreath",
        "Star",
        "Tree",
        "Presents",
        "Silver bells",
        "Little Saint Nick",
        "Here comes Santa Claus",
        "The Christmas song",
        "Deck the halls",
        "Gifts",
        "Winter wonderland",
        "Feliz Navidad",
        "Rockin around the Christmas tree",
        "Frosty",
        "Holly",
        "Ivy",
        "Bells",
        "Carol of the bells",
        "Little drummer boy",
        "Carols",
        "Nativity",
        "Candle",
        "Faith",
        "Joy",
        "Peace",
        "Love",
        "Snow",
        "Sleigh",
        "The North Pole",
        "Frost",
        "Sparkle",
        "Angel",
        "King",
        "Shepherd",
        "Stable",
        "Manger",
        "Let it snow",
        "Noel",
        "Blessing",
        "Unity",
        "Run Rudolph run",
        "Somewhere in my memory",
        "I want a hippopotamus for Christmas",
        "Kindness",
        "Warmth",
        "Glory",
        "Choir",
        "Charity",
        "Celebration",
        "Happiness",
        "Wonder",
        "Miracle",
        "Sleigh ride",
        "God rest ye merry gentlemen",
        "These three kings",
        "Three wisemen",
        "Santa tell me",
        "I saw mommy kissing Santa Claus",
        "All I want for Christmas is you",
        "Cheer",
        "Feast",
        "Tradition",
        "Memory",
        "Prayer",
        "Pudding",
        "Cranberries",
        "Cookies",
        "Gingerbread",
        "O holy night",
        "Up on the housetop",
        "My favorite things",
        "Hot cocoa",
        "Hot chocolate",
        "A marshmallow world",
        "Filled stockings",
        "Grandma got run over by a reindeer",
        "We need a little Christmas",
        "Baby its cold outside",
        "O Christmas tree",
        "Grown up Christmas list",
        "Letters to Santa",
        "Marys boy child",
        "Angels among us",
        "Christmas shoes",
        "Do you want to build a snowman",
        "Santa looked a lot like daddy",
        "Winter dreams",
        "Angels from the realms of glory",
        "Must be Santa",
        "Cocoa",
        "Evergreen",
        "Poinsettia",
        "Skates",
        "Sled",
        "Scarf",
        "Mittens",
        "Hat",
        "Boots",
        "Baby Jesus",
        "Angels we have heard on high",
        "Far far away on Judeas plains",
        "Hark the herald angels sing",
        "I heard the bells on Christmas day",
        "It came upon a midnight clear",
        "Joy to the world",
        "O little town of Bethlehem",
        "Silent night",
        "The first noel",
        "With wondering awe",
        "The Santa Clause",
        "The Christmas story",
        "Rudolph the red nosed reindeer",
        "How the Grinch stole Christmas",
        "The Grinch",
        "Santa baby",
        "Do you hear what I hear",
        "Merry Christmas baby",
        "Blue Christmas",
        "Santa Claus is coming to town",
        "Holly jolly Christmas",
        "It came without ribbons",
        "Swirly twirly gum drops",
        "Buddy the elf",
        "Whats your favorite color",
        "Home Alone",
        "Its a wonderful life",
        "Scrooged",
    ];

    let document = web_sys::window()
        .and_then(|win| win.document())
        .ok_or_else(|| JsValue::from_str("Could not access document"))?;

    let word_display = Rc::new(document.get_element_by_id("word-display").unwrap());
    let attempts_display = Rc::new(document.get_element_by_id("attempts").unwrap());
    let guesses_display = Rc::new(document.get_element_by_id("guesses").unwrap());
    let score_display = Rc::new(document.get_element_by_id("score").unwrap());
    let input_element = Rc::new(
        document
            .get_element_by_id("guess-input")
            .unwrap()
            .dyn_into::<HtmlInputElement>()?,
    );
    let status_display = Rc::new(document.get_element_by_id("status").unwrap());
    let guess_button = Rc::new(document.get_element_by_id("guess-button").unwrap());
    let play_again_button = Rc::new(document.get_element_by_id("play-again-button").unwrap());

    let canvas: HtmlCanvasElement = document
        .get_element_by_id("hangman-canvas")
        .ok_or_else(|| JsValue::from_str("Could not find hangman-canvas"))?
        .dyn_into()?;
    let context = Rc::new(
        canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Could not get 2d context"))?
            .dyn_into::<CanvasRenderingContext2d>()?,
    );

    let game = Rc::new(RefCell::new(Hangman::new(
        &words[rand::random::<usize>() % words.len()],
        6,
    )));

    let update_game_display = {
        let game = game.clone();
        let word_display = word_display.clone();
        let attempts_display = attempts_display.clone();
        let guesses_display = guesses_display.clone();
        let score_display = score_display.clone();
        let context = context.clone();

        move || {
            let game = game.borrow();
            word_display.set_inner_html(&game.display_word());
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
            score_display.set_text_content(Some(&game.get_score()));

            context.clear_rect(0.0, 0.0, 200.0, 250.0);
            draw_hangman_stand(&context);
            let mistakes = game.max_attempts - game.attempts_left();
            draw_hangman_parts(&context, mistakes);
        }
    };
    let update_game_display = Rc::new(update_game_display);

    let handle_guess = {
        let game = game.clone();
        let update_game_display = update_game_display.clone();
        let status_display = status_display.clone();
        let input_element = input_element.clone();
        let play_again_button = play_again_button.clone();

        move || {
            let letter = input_element.value();
            input_element.set_value("");

            let mut g = game.borrow_mut();
            if g.game_over {
                status_display.set_text_content(Some(
                    "Game is over! Click 'Play Again' to start a new game.",
                ));
                return;
            }

            if letter.len() != 1 || !letter.chars().all(|c| c.is_alphabetic()) {
                status_display.set_text_content(Some("Enter a single alphabetic letter!"));
                return;
            }

            let correct = g.guess(letter.clone());
            drop(g);

            if correct {
                status_display.set_text_content(Some("Good guess!"));
            } else {
                status_display.set_text_content(Some("Wrong guess!"));
            }

            update_game_display();

            let mut g = game.borrow_mut();
            if g.is_won() {
                status_display.set_text_content(Some("You won! Want to play again?"));
                play_again_button
                    .set_attribute("style", "display: block;")
                    .unwrap();
            } else if g.is_lost() {
                status_display.set_text_content(Some(&format!(
                    "Game over! The word was '{}'. Want to play again?",
                    g.get_word()
                )));
                play_again_button
                    .set_attribute("style", "display: block;")
                    .unwrap();
            }
        }
    };
    let handle_guess = Rc::new(handle_guess);

    let play_again_closure = {
        let game = game.clone();
        let update_game_display = update_game_display.clone();
        let status_display = status_display.clone();
        let play_again_button = play_again_button.clone();
        let context = context.clone();
        let words = words.clone();

        move || {
            context.clear_rect(0.0, 0.0, 200.0, 250.0);
            draw_hangman_stand(&context);
            let new_word = words[rand::random::<usize>() % words.len()].to_string();
            game.borrow_mut().reset_game(&new_word);
            update_game_display();
            status_display.set_text_content(Some("Game restarted! Start guessing."));
            play_again_button
                .set_attribute("style", "display: none;")
                .unwrap();
        }
    };
    let play_again_closure = Closure::wrap(Box::new(play_again_closure) as Box<dyn FnMut()>);
    play_again_button
        .add_event_listener_with_callback("click", play_again_closure.as_ref().unchecked_ref())?;
    play_again_closure.forget();

    let keydown_closure = {
        let handle_guess = handle_guess.clone();
        Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                handle_guess();
            }
        }) as Box<dyn FnMut(_)>)
    };
    input_element
        .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())?;
    keydown_closure.forget();

    let guess_button_closure = {
        let handle_guess = handle_guess.clone();
        Closure::wrap(Box::new(move || {
            handle_guess();
        }) as Box<dyn FnMut()>)
    };
    guess_button
        .add_event_listener_with_callback("click", guess_button_closure.as_ref().unchecked_ref())?;
    guess_button_closure.forget();

    update_game_display();
    status_display.set_text_content(Some("Game started! Start guessing."));

    Ok(())
}

fn draw_hangman_stand(context: &CanvasRenderingContext2d) {
    context.set_stroke_style(&"#000".into());

    // Base
    context.begin_path();
    context.move_to(50.0, 200.0);
    context.line_to(150.0, 200.0);
    context.stroke();

    // Pole
    context.begin_path();
    context.move_to(100.0, 200.0);
    context.line_to(100.0, 50.0);
    context.stroke();

    // Top beam
    context.begin_path();
    context.move_to(100.0, 50.0);
    context.line_to(140.0, 50.0);
    context.stroke();

    // Rope
    context.begin_path();
    context.move_to(140.0, 50.0);
    context.line_to(140.0, 70.0);
    context.stroke();
}

fn draw_hangman_parts(context: &CanvasRenderingContext2d, mistakes: u8) {
    context.set_stroke_style(&"#000".into());

    if mistakes >= 1 {
        // Head
        context.begin_path();
        context
            .arc(140.0, 90.0, 10.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        context.stroke();
    }

    if mistakes >= 2 {
        // Body
        context.begin_path();
        context.move_to(140.0, 100.0);
        context.line_to(140.0, 140.0);
        context.stroke();
    }

    if mistakes >= 3 {
        // Left Arm
        context.begin_path();
        context.move_to(140.0, 110.0);
        context.line_to(130.0, 130.0);
        context.stroke();
    }

    if mistakes >= 4 {
        // Right Arm
        context.begin_path();
        context.move_to(140.0, 110.0);
        context.line_to(150.0, 130.0);
        context.stroke();
    }

    if mistakes >= 5 {
        // Left Leg
        context.begin_path();
        context.move_to(140.0, 140.0);
        context.line_to(130.0, 170.0);
        context.stroke();
    }

    if mistakes >= 6 {
        // Right Leg
        context.begin_path();
        context.move_to(140.0, 140.0);
        context.line_to(150.0, 170.0);
        context.stroke();
    }
}
