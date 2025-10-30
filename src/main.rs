mod create_player_hand;
mod create_tile_pool;

use crate::{create_player_hand::create_player_hand, create_tile_pool::create_tile_pool};

fn main() {
    println!("Welcome to the Dominoes game, which is coded using the rust programming language!");
    let mut tile_pool: Vec<Vec<i32>> = create_tile_pool();
    let player_hand: Vec<Vec<i32>> = create_player_hand(&mut tile_pool);
}
