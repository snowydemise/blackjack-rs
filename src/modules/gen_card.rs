use rand::prelude::*;

pub fn gen_card() -> i32 {
    let mut rng = rand::rng();
    return rng.random_range(1..=10);
}