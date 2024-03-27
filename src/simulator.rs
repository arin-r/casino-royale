use crate::{
    dealer::Dealer, game_state::GameState, hand::Hand, player::Player, strategy::{Move, Strategy}
};

static BET: i32 = 10;

pub fn play_round<T>(player: &mut Player, dealer: &mut Dealer, game: &mut GameState)
where
    T: Strategy,
{
    player.hands.push_front(Hand::new(game.deal_multiple(2), BET));
    dealer.hand = Hand::new(game.deal_multiple(2), BET);
    let dealer_up_card = &dealer.hand.cards[0];
    let mut i = 0;
    // dbg!(dealer_up_card);
    while i < player.hands.len() {
        // dbg!(&player.hands);
        if player.hands[i].is_standed() || player.hands[i].is_doubled_down() || player.hands[i].get_optimal_total() >= 21 {
            i += 1;
            continue;
        }
        let split_decision = T::split_decision(&player.hands[i], dealer_up_card);
        if split_decision {
            let new_hand = Hand::new(vec![player.hands[i].cards.pop().unwrap()], BET);
            player.hands.push_front(new_hand);
            i = 0;
        } else {
            let move_decision = T::get_move(&player.hands[i], dealer_up_card);
            match move_decision {
                Move::Hit => {
                    player.hands[i].add_card(game.deal());
                }
                Move::Stand => {
                    player.hands[i].stand();
                    i += 1;
                }
                Move::Double=> {
                    player.hands[i].double_down();
                    player.hands[i].add_card(game.deal());
                    i += 1;
                }
            }
        }
    }
    // dbg!(&player.hands);
    while dealer.hand.get_optimal_total() < 17 {
        dealer.hand.add_card(game.deal());
    }
    for hand in &player.hands {
        if hand.get_optimal_total() > 21 {
            player.money -= hand.bet;
        } else if dealer.hand.get_optimal_total() > 21 {
            player.money += hand.bet;
        } else if hand.get_optimal_total() > dealer.hand.get_optimal_total() {
            player.money += hand.bet;
        } else if hand.get_optimal_total() < dealer.hand.get_optimal_total() {
            player.money -= hand.bet;
        }
    }
}

pub fn simulate<T>()
where
    T: Strategy,
{
    let n = 100 * 1000;
    let mut player = Player::new(n * 1000);
    let mut dealer = Dealer::new();
    let mut game = GameState::new();
    // game.shuffle();
    // while game.shoe[game.shoe.len() - 1].rank != game.shoe[game.shoe.len() - 2].rank {
    //     game.shuffle();
    // }
    // dbg!(&game.shoe);
    for _ in 0..n {
        play_round::<T>(&mut player, &mut dealer, &mut game);
        player.hands.clear();
        dealer.clear_hand();
    }
    dbg!(&player);
    let profit = player.money - n * 1000;
    let relative_profit = profit as f64  * 100 as f64 / (n * 10) as f64;
    dbg!(profit, relative_profit);
}
