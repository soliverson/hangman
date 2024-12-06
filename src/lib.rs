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
    wins: u32,
    losses: u32,
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
                    " ".to_string() // Space directly for multi-word phrases
                } else if self.guesses.contains(&c.to_string()) {
                    c.to_string()
                } else {
                    "_".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("")
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
        if self
            .word
            .chars()
            .filter(|&c| c != ' ')
            .all(|c| self.guesses.contains(&c.to_string()))
        {
            self.wins += 1;
            true
        } else {
            false
        }
    }

    pub fn is_lost(&mut self) -> bool {
        if self.attempts_left() == 0 {
            self.losses += 1;
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
        "Peace on Earth",
        "Goodwill",
        "Nativity",
        "Shepherds",
        "Angels",
        "Wise Men",
        "Bethlehem",
        "Frankincense",
        "Myrrh",
        "Gold",
        "Immanuel",
        "Caroling",
        "Eggnog",
        "Snowflakes",
        "Icicles",
        "Winter Wonderland",
        "Snowman",
        "Christmas Lights",
        "Stocking Stuffers",
        "Elf on the Shelf",
        "Ornaments",
        "Tinsel",
        "Garland",
        "Mistletoe",
        "Christmas Cookies",
        "Hot Cocoa",
        "Gingerbread House",
        "Holiday Spirit",
        "North Pole",
        "Rudolph the Red Nosed Reindeer",
        "Christmas Eve",
        "Yuletide",
        "Jingle Bells",
        "White Christmas",
        "December",
        "Poinsettia",
        "Noel",
        "Christmas Wreath",
        "Christmas Cheer",
        "Sleigh Bells",
        "Saint Nicholas",
        "Christmas Morning",
        "Wrapping Paper",
        "Gift Exchange",
        "Snowball Fight",
        "Christmas Stocking",
        "Winter Solstice",
        "Holiday Greetings",
        "Christmas Dinner",
        "Joy to the World",
        "Hark the Herald Angels Sing",
        "O Holy Night",
        "Deck the Halls",
        "O Little Town of Bethlehem",
        "Silent Snowfall",
        "Cozy Fireplaces",
        "Evergreen Trees",
        "Winter Nights",
        "Snow Angels",
        "Happy Holidays",
        "Merry Christmas",
        "Christmas Star",
        "Holiday Traditions",
        "Advent Calendar",
        "Christmas Countdown",
        "Festive Music",
        "Twinkling Lights",
        "North Star",
        "Partridge in a Pear Tree",
        "Christmas Carols",
        "Sugarplum Dreams",
        "Christmas Spirit",
        "Yuletide Joy",
        "Holiday Feast",
        "Newborn King",
        "Bethlehem Star",
        "Three Kings",
        "Family Gatherings",
        "Holiday Baking",
        "Season of Giving",
        "Cozy Sweaters",
        "Crackling Fire",
        "Holiday Cards",
        "Gift Wrapping",
        "Snow Covered Streets",
        "Christmas Parade",
        "Saint Nick",
        "Midnight Mass",
        "Winter Mittens",
        "Holiday Magic",
        "Warm Blankets",
        "Christmas Elves",
        "Chimney Stockings",
        "Christmas Miracle",
        "Christmas Songs",
        "Silver Bells",
        "Chestnuts Roasting",
        "Holiday Joy",
        "Holiday Wishes",
        "Sleigh Rides",
        "Christmas Bells",
        "Feliz Navidad",
        "Nativity Scene",
        "Christmas Angel",
        "Little Drummer Boy",
        "Holy Night",
        "Family Traditions",
        "Santas Workshop",
        "Snowy Mountains",
        "Reindeer Games",
        "Christmas Cheer",
        "Warm Hugs",
        "Winter Scarves",
        "Festive Candles",
        "Christmas Decorations",
        "Yuletide Blessings",
        "Seasons Greetings",
        "Holiday Lights",
        "Frozen Lake",
        "Christmas Ornaments",
        "Frosty Nights",
        "Holiday Gathering",
        "Christmas Hymns",
        "Christmas Morning Joy",
        "Magic of Christmas",
        "Bright Stars",
        "Winter Winds",
        "Church Bells",
        "Midnight Snow",
        "Christmas Village",
        "Cozy Nights",
        "Silent Prayers",
        "Festive Treats",
        "Holiday Celebrations",
        "Warm Cider",
        "Happy Tidings",
        "Golden Bells",
        "Santas Sleigh",
        "Holiday Memories",
        "Reindeer Antlers",
        "Starry Night",
        "Christmas Feast",
        "Family Laughter",
        "Holy Family",
        "Holiday Cheer",
        "Christmas Market",
        "Yuletide Memories",
        "Christmas Presents",
        "Warm Wishes",
        "Winter Joy",
        "Holiday Fun",
        "Christmas Spirit",
        "Snow Globe",
        "Christmas Dream",
        "Chimney Smoke",
        "Holiday Story",
        "Starry Skies",
        "Christmas Blessings",
        "Church Choir",
        "Winter Warmth",
        "December Snow",
        "Holiday Treats",
        "Christmas Prayers",
        "Holly Berries",
        "Christmas Songs",
        "Cozy Fire",
        "Christmas Wonder",
        "Holiday Bells",
        "Gift of Love",
        "Winter Magic",
        "Snowy Nights",
        "Christmas Wishes",
        "Christmas Candles",
        "Festive Wreath",
        "Christmas Magic",
        "December Nights",
        "Christmas Peace",
        "Christmas Lullaby",
        "Christmas Time",
        "Winter Bliss",
        "Christmas Rejoice",
        "Christmas Glory",
        "Holiday Love",
        "Holiday Hope",
        "Snow Covered Hills",
        "Bright Candlelight",
        "Shimmering Stars",
        "Christmas Garland",
        "Holiday Comfort",
        "Gift of Giving",
        "Christmas Tidings",
        "Silent Worship",
        "Holy Hymns",
        "Christmas Blessing",
        "Winter Twilight",
        "Warm Hearth",
        "Angel Choir",
        "Christmas Radiance",
        "Holiday Glow",
        "Snowy Village",
        "Warm Embrace",
        "Festive Decor",
        "Christmas Serenity",
        "Christmas Glow",
        "Santas Reindeer",
        "Winter Scene",
        "Christmas Chimes",
        "Silent Snow",
        "Holiday Glow",
        "Christmas Eve Prayer",
        "Snowy Night Sky",
        "Family Togetherness",
        "Christmas Tree Glow",
        "Holiday Joyful Hearts",
        "Yuletide Splendor",
        "Snowfall Silence",
        "Christmas Twilight",
        "Santas Arrival",
        "Reindeer Flight",
        "Peaceful Holiday",
        "Blessed Christmas",
        "Christmas Harmony",
        "Christmas Gratitude",
        "Christmas Spirit Shared",
        "Angelic Tidings",
        "Christmas Candle light",
        "Snow Kissed Morning",
        "Holy Christmas",
        "Winter Solstice Glow",
        "Joyous Holiday",
        "Gift of Togetherness",
        "Christmas Bliss",
        "Santas Bag of Gifts",
        "Magic of Giving",
        "Family Warmth",
        "Holiday Star",
        "Christmas Eve Magic",
        "Celebration of Love",
        "Christmas Embrace",
        "Silent Blessing",
        "Golden Star",
        "Holiday Whisper",
        "Christmas Angel Glow",
        "Bright Christmas Day",
        "Winters Calm",
        "Holiday Unity",
        "Faithful Christmas",
        "Frosted Pinecones",
        "Yuletide Laughter",
        "Radiant Noel",
        "Holiday Miracles",
        "Christmas Reverence",
        "Christmas Tranquility",
        "Sparkling Tinsel",
        "Santas Laugh",
        "Joyful Snowmen",
        "Christmas Reflection",
        "Christmas Anticipation",
        "Seasonal Splendor",
        "Snow Kissed Tree",
        "Santas Smile",
        "Gift of Gratitude",
        "Silent Christmas Star",
        "Christmas Hope Eternal",
        "Yuletide Blessing Bright",
        "Christmas Comfort Shared",
        "Golden Glow of Christmas",
        "Reindeer Sleigh Bells",
        "Christmas Morning Delight",
        "Blessings in December",
        "Christmas Eve Star",
        "Magic in the Air",
        "Shimmering Icicles",
        "Christmas Gratitude Overflow",
        "Yuletide Stars Shine",
        "Blessed Joy of Christmas",
        "Christmas Peacefulness",
        "Frosty Windowpanes",
        "Holiday Kindness Shared",
        "Christmas Hymns Sung",
        "Radiant Glow of Noel",
        "Winters Bliss",
        "Christmas Songbird",
        "Christmas Eve Calm",
        "Angelic Christmas Songs",
        "Starry Night Rejoice",
        "Peaceful Noel",
        "Christmas Wishes Fulfilled",
        "Christmas Lights Twinkle",
        "Winter Serenade",
        "Golden Ornaments Shine",
        "Faithful Blessings of Noel",
        "Christmas Warmth Overflowing",
        "Frosted Christmas Night",
        "Angels We Have Heard on High",
        "Golden Christmas Glow",
        "Winters Silent Prayer",
        "Yuletide Hopes Renewed",
        "Radiant Manger Scene",
        "Frosted Trees of Christmas",
        "Silent Wonders of Christmas",
        "Golden Star of Noel",
        "Christmas Hearts United",
        "Yuletide Radiance",
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

    let game = Rc::new(RefCell::new(Hangman::new(
        &words[rand::random::<usize>() % words.len()],
        5,
    )));

    let update_game_display = Rc::new({
        let game = game.clone();
        let word_display = word_display.clone();
        let attempts_display = attempts_display.clone();
        let guesses_display = guesses_display.clone();
        let score_display = score_display.clone();

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
            score_display.set_text_content(Some(&game.get_score()));
        }
    });

    let handle_guess = Rc::new({
        let game = game.clone();
        let update_game_display = update_game_display.clone();
        let status_display = status_display.clone();
        let input_element = input_element.clone();
        let play_again_button = play_again_button.clone();

        move || {
            let letter = input_element.value();
            input_element.set_value("");

            if letter.len() != 1 || !letter.chars().all(|c| c.is_alphabetic()) {
                status_display.set_text_content(Some("Enter a single alphabetic letter!"));
                return;
            }

            if game.borrow_mut().guess(letter.clone()) {
                status_display.set_text_content(Some("Good guess!"));
            } else {
                status_display.set_text_content(Some("Wrong guess!"));
            }

            update_game_display();

            let mut game_ref = game.borrow_mut();
            if game_ref.is_won() {
                status_display.set_text_content(Some("You won! Want to play again?"));
                play_again_button
                    .set_attribute("style", "display: block;")
                    .unwrap();
            } else if game_ref.is_lost() {
                status_display.set_text_content(Some(&format!(
                    "Game over! The word was '{}'. Want to play again?",
                    game_ref.get_word()
                )));
                play_again_button
                    .set_attribute("style", "display: block;")
                    .unwrap();
            }
        }
    });

    let play_again_closure = Closure::wrap(Box::new({
        let game = game.clone();
        let update_game_display = update_game_display.clone();
        let status_display = status_display.clone();
        let play_again_button = play_again_button.clone();

        move || {
            let new_word = words[rand::random::<usize>() % words.len()].to_string();
            game.borrow_mut().reset_game(&new_word);
            update_game_display();
            status_display.set_text_content(Some("Game restarted! Start guessing."));
            play_again_button
                .set_attribute("style", "display: none;")
                .unwrap();
        }
    }) as Box<dyn FnMut()>);

    play_again_button
        .add_event_listener_with_callback("click", play_again_closure.as_ref().unchecked_ref())?;
    play_again_closure.forget();

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

    update_game_display();
    status_display.set_text_content(Some("Game started! Start guessing."));

    Ok(())
}
