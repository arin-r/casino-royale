use core::panic;

use crate::{
    card::{Card, Rank},
    hand::Hand,
};

use super::{Move, Strategy};

pub struct BasicStrategy {}

#[derive(Debug, PartialEq)]
pub enum HandType {
    Hard(i32),
    Soft(i32),
}

impl BasicStrategy {
    pub fn hand_type(hand: &Hand) -> HandType {
        let hand = hand.get_cards();
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
        if total > 21 {
            HandType::Hard(total)
        } else if aces > 0 {
            HandType::Soft(total)
        } else {
            HandType::Hard(total)
        }
    }
}

impl Strategy for BasicStrategy {
    fn split_decision(hand: &Hand, dealer_up_card: &Card) -> bool {
        let player_cards = hand.get_cards();

        if player_cards.len() != 2 {
            return false;
        }

        if player_cards[0].rank != player_cards[1].rank {
            return false;
        }
        match player_cards[0].rank {
            Rank::Ace => true,
            Rank::Num(2) | Rank::Num(3) => match dealer_up_card.rank {
                Rank::Num(2)
                | Rank::Num(3)
                | Rank::Num(4)
                | Rank::Num(5)
                | Rank::Num(6)
                | Rank::Num(7) => true,
                _ => false,
            },
            Rank::Num(4) => match dealer_up_card.rank {
                Rank::Num(5) | Rank::Num(6) => true,
                _ => false,
            },
            Rank::Num(5) => false,
            Rank::Num(6) => match dealer_up_card.rank {
                Rank::Num(2) | Rank::Num(3) | Rank::Num(4) | Rank::Num(5) | Rank::Num(6) => true,
                _ => false,
            },
            Rank::Num(7) => match dealer_up_card.rank {
                Rank::Num(2)
                | Rank::Num(3)
                | Rank::Num(4)
                | Rank::Num(5)
                | Rank::Num(6)
                | Rank::Num(7) => true,
                _ => false,
            },
            Rank::Num(8) => true,
            Rank::Num(9) => match dealer_up_card.rank {
                Rank::Num(7) | Rank::Num(10) | Rank::Face(_) | Rank::Ace => false,
                _ => true,
            },
            Rank::Num(10) | Rank::Face(_) => false,
            _ => {
                panic!("Invalid dealer up card")
            }
        }
    }

    fn get_move(hand: &Hand, dealer_up_card: &Card) -> Move {
        let player_cards = hand.get_cards();
        let hand_type = Self::hand_type(hand);
        match hand_type {
            HandType::Hard(n) => match n {
                21 | 20 | 19 | 18 | 17 => Move::Stand,
                16 | 15 | 14 | 13 => match dealer_up_card.rank {
                    Rank::Num(2) | Rank::Num(3) | Rank::Num(4) | Rank::Num(5) | Rank::Num(6) => {
                        Move::Stand
                    }
                    _ => Move::Hit,
                },
                12 => match dealer_up_card.rank {
                    Rank::Num(4) | Rank::Num(5) | Rank::Num(6) => Move::Stand,
                    _ => Move::Hit,
                },
                11 => Move::Double,
                10 => match dealer_up_card.rank {
                    Rank::Ace | Rank::Num(10) | Rank::Face(_) => Move::Hit,
                    _ => Move::Double,
                },
                9 => match dealer_up_card.rank {
                    Rank::Num(3) | Rank::Num(4) | Rank::Num(5) | Rank::Num(6) => Move::Double,
                    _ => Move::Hit,
                },
                8 | 7 | 6 | 5 | 4 | 3 | 2 => Move::Hit,
                _ => {
                    panic!("Invalid hand type")
                }
            },
            HandType::Soft(n) => match n {
                20 => Move::Stand,
                19 => match dealer_up_card.rank {
                    Rank::Num(6) => Move::Double,
                    _ => Move::Stand,
                },
                18 => match dealer_up_card.rank {
                    Rank::Num(2) | Rank::Num(3) | Rank::Num(4) | Rank::Num(5) | Rank::Num(6) => {
                        Move::Double
                    }
                    Rank::Num(7) | Rank::Num(8) => Move::Stand,
                    _ => Move::Hit,
                },
                17 => match dealer_up_card.rank {
                    Rank::Num(3) | Rank::Num(4) | Rank::Num(5) | Rank::Num(6) => Move::Double,
                    _ => Move::Hit,
                },
                16 => match dealer_up_card.rank {
                    Rank::Num(4) | Rank::Num(5) | Rank::Num(6) => Move::Double,
                    _ => Move::Hit,
                },
                15 => match dealer_up_card.rank {
                    Rank::Num(4) | Rank::Num(5) | Rank::Num(6) => Move::Double,
                    _ => Move::Hit,
                },
                14 | 13 => match dealer_up_card.rank {
                    Rank::Num(5) | Rank::Num(6) => Move::Double,
                    _ => Move::Hit,
                },
                12 => panic!("Soft 12 is not possible, cause we will always split a pair of aces"),
                11 => {
                    Move::Hit
                }
                _ => {
                    panic!(
                        "Invalid hand type, cards = {:?}, hand_type = {:?}",
                        &player_cards, &hand_type
                    );
                }
            },
        }
    }
}
