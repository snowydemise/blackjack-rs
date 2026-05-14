use colour::{blue_ln, cyan_ln, grey_ln, green_ln, red_ln};
use std::io;

use crate::modules::gen_card::{self, gen_card};

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

    grey_ln!(
        "Your hand: {}",
        player_cards
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    loop {
        println!();

        cyan_ln!("Please choose your input: H - Hit :: S - Stand");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("H") {
            let new_player_card = gen_card();
            let new_dealer_card = gen_card();

            player_cards.push(new_player_card);
            dealer_cards.push(new_dealer_card);

            cyan_ln!(
                "New hand: {}",
                player_cards
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            );

            let player_total: i32 = player_cards.iter().sum();
            let dealer_total: i32 = dealer_cards.iter().sum();

            grey_ln!("Your total: {}", player_total);

            // Player bust
            if player_total > 21 {
                red_ln!("Bust! You lost.");
                return false;
            }

            // Dealer bust
            if dealer_total > 21 {
                green_ln!("Congrats! Dealer busted with {}", dealer_total);
                return true;
            }

            continue;
        }

        if input.trim().eq_ignore_ascii_case("S") {
            let player_total: i32 = player_cards.iter().sum();
            let dealer_total: i32 = dealer_cards.iter().sum();

            if player_total >= dealer_total {
                green_ln!("Congrats! You won");
                return true;
            } else {
                red_ln!("Sorry! You lost.. the dealer had {}", dealer_total);
                return false;
            }
        }

        red_ln!("Invalid input.");
    }
}

pub fn play_game() {
    cyan_ln!("Dealing cards...");
    mini_game();
}