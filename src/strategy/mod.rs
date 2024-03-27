pub mod basic_strategy;
use crate::{card::Card, hand::Hand};

pub enum Move {
    Hit,
    Stand,
    Double,
}

pub trait Strategy {
    /// Returns true if pair of cards should be split and false otherwise
    fn split_decision(hand: &Hand, dealer_up_card: &Card) -> bool;
    /// Returns the move to be played
    fn get_move(hand: &Hand, dealer_up_card: &Card) -> Move;
}