use crate::card::{Card, Rank};
#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    doubled_down: bool,
    is_standed: bool,
    pub bet: i32,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bet: i32) -> Self {
        Self {
            cards,
            doubled_down: false,
            is_standed: false,
            bet,
        }
    }
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn add_cards(&mut self, cards: Vec<Card>) {
        self.cards.extend(cards);
    }
    /// Function to double down on a hand.
    /// Before calling this function the player should have enough money to double the bet.
    /// This function should only be called once per hand.
    pub fn double_down(&mut self) {
        self.doubled_down = true;
        self.bet *= 2;
    }
    pub fn is_standed(&self) -> bool {
        self.is_standed
    }
    pub fn stand(&mut self) {
        self.is_standed = true;
    }
    pub fn is_doubled_down(&self) -> bool {
        self.doubled_down
    }
    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }
    pub fn get_optimal_total(&self) -> i32 {
        let hand = self.get_cards();
        let mut total = 0;
        let mut aces = 0;
        for card in hand {
            match card.rank {
                Rank::Num(n) => total += n,
                Rank::Face(_) => total += 10,
                Rank::Ace => {
                    total += 11;
                    aces += 1;
                }
            }
        }
        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }
        total
    }
    /// check if the hand is a blackjack
    pub fn is_blackjack(&self) -> bool {
        self.get_optimal_total() == 21
    }
}
