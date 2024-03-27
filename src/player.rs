use std::collections::VecDeque;

use crate::hand::Hand;

#[derive(Debug)]
pub struct Player {
    /// The player may have multiple hands, so we store them in a vector.
    /// Unlike the dealer, the player can have multiple hands at the same time hence a vector is used
    /// Initially, the player has no hands, so the vector is empty. The design difference between 
    /// the dealer and the player is that when the dealer does not have an hand the Option::None variant is used
    /// whereas, when the player does not have a hand, an empty vector is used.
    pub hands: VecDeque<Hand>,
    /// The player's money
    pub money: i32,
}

impl Player {
    pub fn new(initial_money: i32) -> Self {
        Self {
            hands: VecDeque::new(),
            money: initial_money,
        }
    }
}