use colour::{blue_ln, cyan_ln, grey_ln, green_ln, red_ln};
use std::io;

use crate::modules::gen_card::{self, gen_card};

fn hit(_dealer_cards: &mut Vec<i32>, _player_cards: &mut Vec<i32>) {

}

fn mini_game() -> bool {
    let dealer_card_1 = gen_card::gen_card();
    let dealer_card_2 = gen_card::gen_card();
    let mut dealer_cards = vec![dealer_card_1, dealer_card_2];

    let player_card_1 = gen_card::gen_card();
    let player_card_2 = gen_card::gen_card();
    let mut player_cards = vec![player_card_1, player_card_2];

    println!();
    blue_ln!("Dealer's card: {}", dealer_card_1);
    println!();
    grey_ln!("Your hand: First Card: {}, Second Card: {}", player_card_1, player_card_2);

    let mut input = String::new();

    cyan_ln!("Please choose your input: H - Hit :: S - Stand");
    println!();

    io::stdin()
        .read_line(&mut input) 
        .expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("H") {
        let new_dealer_card = gen_card();
        let new_player_card = gen_card();

        if dealer_cards.iter().sum::<i32>() > 21 {
            green_ln!("Congrats! You won, the dealer had {}", dealer_cards.iter().sum::<i32>());
            return true
        };

        player_cards.push(new_player_card);
        dealer_cards.push(new_dealer_card);

        if player_cards.iter().sum::<i32>() >= dealer_cards.iter().sum::<i32>() {
            cyan_ln!("New hand: {}", player_cards.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));
        } else {
            red_ln!("Sorry! You lost.. the dealer had {}", dealer_cards.iter().sum::<i32>());
            return false
        }
    };

    if input.trim().eq_ignore_ascii_case("S") {
        if player_cards.iter().sum::<i32>() >= dealer_cards.iter().sum::<i32>() {
            green_ln!("Congrats! You won");
            return true
        } else {
            red_ln!("Sorry! You lost.. the dealer had {}", dealer_cards.iter().sum::<i32>());
            return false
        };
    };
    return  false;
}

pub fn play_game() {
    cyan_ln!("Dealing cards...");
    mini_game();
}