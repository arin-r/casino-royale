use crate:: card::{Card, Face, Rank, Suit};

pub struct GameState {
    // Temporary public for testing
    pub shoe: Vec<Card>,
    pub count: i32,
    shoe_size: i32,
}

impl GameState {
    fn new_shoe(shoe_size: i32) -> Vec<Card> {
        let mut deck = Vec::new();
        for _ in 0..shoe_size {
            for suit in vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
                for rank in vec![
                    Rank::Ace,
                    Rank::Num(2),
                    Rank::Num(3),
                    Rank::Num(4),
                    Rank::Num(5),
                    Rank::Num(6),
                    Rank::Num(7),
                    Rank::Num(8),
                    Rank::Num(9),
                    Rank::Num(10),
                    Rank::Face(Face::Jack),
                    Rank::Face(Face::Queen),
                    Rank::Face(Face::King),
                ] {
                    deck.push(Card::new(suit.clone(), rank.clone()));
                }
            }
        }
        deck
    }
    /// Creates a new game state with a new un-shuffled shoe
    pub fn new() -> Self {
        Self {
            shoe: Self::new_shoe(6),
            count: 0,
            shoe_size: 6,
        }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut rng = thread_rng();
        self.shoe.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Card {
        let card = self.shoe.pop().expect("No more cards to deal");

        if self.shoe.len() < self.shoe_size as usize * 52 / 2 {
            self.shoe = Self::new_shoe(self.shoe_size);
            self.shuffle();
        }

        card
    }

    pub fn deal_multiple(&mut self, n: usize) -> Vec<Card> {
        let mut cards = Vec::new();
        for _ in 0..n {
            cards.push(self.shoe.pop().expect("No more cards to deal"));
        }

        if self.shoe.len() < self.shoe_size as usize * 52 / 2 {
            self.shoe = Self::new_shoe(self.shoe_size);
            self.shuffle();
        }

        cards
    }
}
