use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;

struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        let suits = vec!["Spades", "Diamonds", "Hearts", "Clubs"];
        
        let values = vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

        let mut cards = Vec::new();

        for suit in &suits {
            for value in &values {
                cards.push(format!("{} of {}", value, suit));
            }
        }

        Deck { cards }
    }

    fn print (&self) {
        for (i, card) in self.cards.iter().enumerate() {
            println!("{}: {}", i, card);
        }  
    }

    fn deal(&mut self, hand_size: usize) -> (Vec<String>, Vec<String>) {
        let hand : Vec<String> = self.cards.drain(0..hand_size.min(self.cards.len())).collect();
        (hand.clone(), self.cards.clone())
    }

    fn to_string(&self) -> String {
        self.cards.join(",")
    }

    fn save_to_file(&self, filename: &str) {
        fs::write(filename, self.to_string()).expect("Unable to write file");
    }

    fn load_from_file(filename: &str) -> Self {
        let content = fs::read_to_string(filename).expect("Unable to read file");
        let cards: Vec<String> = content.split(',').map(|s| s.to_string()).collect();
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    deck.print();

    let (hand, remaining) = deck.deal(5);
    println!("\nDealt hand: {:?}", hand);
    println!("Remaining cards: {}", remaining.len());

    deck.save_to_file("deck.txt");
    let loaded_deck = Deck::load_from_file("deck.txt");
    println!("\nLoaded deck: ");
    loaded_deck.print();
}