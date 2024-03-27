use crate::  hand::Hand ;

pub struct Dealer {
    /// The dealer may or may not have a hand, depending on the state of the game.
    /// Just like the player, the dealer starts with no hand.
    /// When the dealer does not have a hand it is represented by None.
    pub hand: Hand,
}

impl Dealer {
    pub fn new() -> Self {
        Dealer { hand: Hand::new(Vec::new(), 0) }
    }
    pub fn clear_hand(&mut self) {
        self.hand = Hand::new(Vec::new(), 0);
    }
}