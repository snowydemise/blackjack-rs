use colour::{blue_ln, cyan_ln, grey_ln, green_ln, red_ln};
use std::io;

use crate::modules::gen_card::{self};

fn mini_game() -> bool {
    let dealer_card_1 = gen_card::gen_card();
    let dealer_card_2 = gen_card::gen_card();
    let dealer_total = dealer_card_1 + dealer_card_2;

    let player_card_1 = gen_card::gen_card();
    let player_card_2 = gen_card::gen_card();
    let player_total = player_card_1 + player_card_2;

    println!();
    blue_ln!("Dealer's card: {}", dealer_card_1);
    println!();
    grey_ln!("Your hand: First Card: {}, Second Card: {}", player_card_1, player_card_2);

    let mut input = String::new();

    cyan_ln!("Please choose your input: H - Hit :: S - Stand");

    io::stdin()
        .read_line(&mut input) 
        .expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("S") {
        if player_total >= dealer_total {
            green_ln!("Congrats! You won");
            return true
        } else {
            red_ln!("Sorry! You lost.. the dealer had {}", dealer_total);
            return false
        };
    };
    return  false;
}

pub fn play_game() {
    cyan_ln!("Dealing cards...");
    mini_game();
}