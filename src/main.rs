mod modules;
use colour::{white_ln_bold};

fn main() {
    white_ln_bold!("Welcome to blackjack-rs, developed by snowydemise!");
    println!();
    
    modules::play_game::play_game();
}
