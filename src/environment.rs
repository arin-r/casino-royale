#![allow(dead_code)]

enum DealerRule {
    HitSoft17,
    StandSoft17,
}

pub struct Environment {
    /// dealer rules for hitting or standing on soft 17
    dealer_rules: DealerRule,
    /// shoe size in number of decks
    num_decks: u8,
    /// maximum splits
    max_splits: u8,
    /// allow double after split
    double_after_split: bool,
    /// allow resplitting aces
    resplit_aces: bool,
}