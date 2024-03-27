pub mod player;
pub mod card;
pub mod dealer;
pub mod environment;
pub mod game_state;
pub mod strategy;
pub mod hand;
pub mod simulator;

#[cfg(test)]
mod tests {
    use crate::{card::Rank, hand::Hand, strategy::basic_strategy::HandType};
    #[allow(unused_imports)]
    use crate::{card::{Card, Suit}, strategy::basic_strategy::BasicStrategy};

    #[test]
    fn hand_type_hard_1() {
        let hand = vec![
            Card::new(Suit::Hearts, Rank::Num(10)),
            Card::new(Suit::Hearts, Rank::Num(10)),
        ];
        let hand = Hand::new(hand, 10);
        assert_eq!(BasicStrategy::hand_type(&hand), HandType::Hard(20));
    }
}

