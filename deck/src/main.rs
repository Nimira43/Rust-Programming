use std::fs;
use rand::seq::SliceRandom
use rand::thread_rng

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
                cards.push(format("{} of {}", value, suit));
            }
        }

        Deck { cards }
    }

    fn print (&self) {
        for (i, card) in self.cards.iter().enumerate() {
            println!("{}: {}", i, card);
        }  
    }

    fn deal(&mut self, hand_size: usize) -> (Vec<String>0, Vec<String>) {
        let hand : Vec<String> = self.cards.drain(0..hand_size.min(self.cards.len())).collect();
        (hand.clone(), self.cards.clone())
    }

    fn to_string(&self) -> String {
        self.cards.join(",");
    }

    
}