#[derive(Debug, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Rank {
    Ace,
    // can be changed to u8
    Num(i32),
    Face(Face),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Face {
    Jack,
    Queen,
    King,
}

// #[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.suit {
            Suit::Clubs => write!(f, "C"),
            Suit::Diamonds => write!(f, "D"),
            Suit::Hearts => write!(f, "H"),
            Suit::Spades => write!(f, "S"),
        }?;
        match &self.rank {
            Rank::Ace => write!(f, "A"),
            Rank::Num(n) => {
                match n {
                    10 => write!(f, "T"),
                    _ => write!(f, "{}", n),
                }
            },
            Rank::Face(face) => match face {
                Face::Jack => write!(f, "J"),
                Face::Queen => write!(f, "Q"),
                Face::King => write!(f, "K"),
            },
        }
    }
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.suit {
            Suit::Clubs => write!(f, "C"),
            Suit::Diamonds => write!(f, "D"),
            Suit::Hearts => write!(f, "H"),
            Suit::Spades => write!(f, "S"),
        }?;
        match &self.rank {
            Rank::Ace => write!(f, "A"),
            Rank::Num(n) => {
                match n {
                    10 => write!(f, "T"),
                    _ => write!(f, "{}", n),
                }
            },
            Rank::Face(face) => match face {
                Face::Jack => write!(f, "J"),
                Face::Queen => write!(f, "Q"),
                Face::King => write!(f, "K"),
            },
        }
    }
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        if let Rank::Num(n) = rank {
            if n < 2 || n > 10 {
                panic!("Invalid card rank");
            }
        }
        Self { suit, rank }
    }
}