use core::fmt;
use std::io;
use serde::{Serialize, Deserialize};
use rand;
use rand::seq::SliceRandom;
use ask_gemini::Gemini;
use std::fmt::Write;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Suit::Spades => "Spades",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Card {
    Number(u8, Suit),
    Jack(Suit),
    Queen(Suit),
    King(Suit),
    WhiteJoker,
    BlackJoker,
}

impl Card {
    fn meaning(&self) -> String {
        match self {
            Card::Number(value, suit) => {
                let number_meaning = match value {
                    1 => "New beginnings, potential",
                    2 => "Duality, choices, partnerships",
                    3 => "Creativity, communication, self-expression",
                    4 => "Stability, foundation, security",
                    5 => "Change, transition, unexpected developments",
                    6 => "Harmony, balance, responsibility",
                    7 => "Mystery, introspection, seeking knowledge",
                    8 => "Transformation, power, abundance",
                    9 => "Completion, endings, humanitarianism",
                    10 => "Fulfillment, achievement, culmination",
                    _ => panic!("Number {}", value.to_string()),
                };
                format!("{} of {}: {}", number_meaning, suit.to_string(), number_meaning)
            }
            Card::Jack(suit) => format!("Jack of {}: Initiative, new ideas, adaptability", suit.to_string()),
            Card::Queen(suit) => format!("Queen of {}: Nurturing, intuition, emotional mastery", suit.to_string()),
            Card::King(suit) => format!("King of {}: Leadership, authority, stability", suit.to_string()),
            Card::WhiteJoker => "Powerful positive force, hidden potential, a guiding light within the dream".to_string(),
            Card::BlackJoker => "Significant obstacle, fear, or negative influence in the dream".to_string(),
        }
    }
}

pub fn create_54_standard_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    //Add Spades cards
    for value in 1..=10 {
        deck.push(Card::Number(value, Suit::Spades))
    }
    deck.push(Card::Jack(Suit::Spades));
    deck.push(Card::Queen(Suit::Spades));
    deck.push(Card::King(Suit::Spades));

    //Add Hearts cards
    for value in 1..=10 {
        deck.push(Card::Number(value, Suit::Hearts))
    }
    deck.push(Card::Jack(Suit::Hearts));
    deck.push(Card::Queen(Suit::Hearts));
    deck.push(Card::King(Suit::Hearts));

    //Add Diamonds cards
    for value in 1..=10 {
        deck.push(Card::Number(value, Suit::Diamonds))
    }
    deck.push(Card::Jack(Suit::Diamonds));
    deck.push(Card::Queen(Suit::Diamonds));
    deck.push(Card::King(Suit::Diamonds));

    //Add Clubs cards
    for value in 1..=10 {
        deck.push(Card::Number(value, Suit::Clubs))
    }
    deck.push(Card::Jack(Suit::Clubs));
    deck.push(Card::Queen(Suit::Clubs));
    deck.push(Card::King(Suit::Clubs));
    
    //Add Jokers
    deck.push(Card::WhiteJoker);
    deck.push(Card::BlackJoker);

    deck
}

pub fn shuffle_deck(deck: &mut Vec<Card>) {
    deck.shuffle(&mut rand::thread_rng());
}

pub fn draw_three_cards(deck: &mut Vec<Card>) -> Vec<Card> {
    if deck.len() < 3 {
        panic!("Deck has less than 3 cards. Cannot draw 3 cards.");
    }

    deck.shuffle(&mut rand::thread_rng());
    deck.drain(..3).collect()
}

fn format_reading(reading: &[String]) -> String {
    let mut formatted_output = String::new();
    for line in reading {
      write!(formatted_output, "{}", line).unwrap();
      write!(formatted_output, "\n").unwrap();
    }
    formatted_output.trim_end().to_string()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let gemini_api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set");

    println!("[INFO] Welcome to Vadar Fortune Teller!");
    println!("[INFO] Please let me know your name: ");
    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("[ERROR] Failed to get username");

    let username: String = username.trim().parse()
        .expect("[ERROR] Please let me know your name first!");

    println!("[INFO] Welcome onboard, {}!", username); 

    let mut deck = create_54_standard_deck();
    let drawn_cards = draw_three_cards(&mut deck);
    println!("[INFO] Drawn cards:");

    let mut keywords: Vec<String> = Vec::new();

    for card in drawn_cards {
        println!("==> {:?}", card);
        println!("{}", card.meaning());

        keywords.push(card.meaning());
    }

    // println!("{:?}", keywords);

    let prompt: String = format!("write fortune teller for these keywords the past: '{}', \
        the present: '{}' \
        and the future: '{}'", 
        keywords[0], 
        keywords[1], 
        keywords[2]
    );
    
    // println!("PROMPTS = {prompt}");

    let gemini = Gemini::new(Some(&gemini_api_key), None);
    match gemini.ask(&prompt).await {
        Ok(response) => {
            //println!("[INFO] Signal from the universe: \n {:?}", response);
            println!("[INFO] Signal from the universe: \n");
            let formatted_reading = format_reading(&response);
            println!("{}", formatted_reading)
        },
        Err(e) => eprintln!("Error: {}", e),
    }


    println!("[INFO] Thanks for using my product!");
    println!("[INFO] Any comments? ");

    let mut comment = String::new();

    io::stdin()
        .read_line(&mut comment)
        .expect("[ERROR] Failed to get comment");

    let comment: String = comment.trim().parse()
        .expect("[ERROR] Please let me know your comment!");

    println!("[INFO] Your review is {}!", comment); 

}