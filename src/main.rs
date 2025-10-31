mod create_player_hand;
mod create_tile_pool;

use crate::{create_player_hand::create_player_hand, create_tile_pool::create_tile_pool};

fn main() {
    println!("Welcome to the Dominoes game, which is coded using the rust programming language!");
    let mut tile_pool: Vec<[i32; 2]> = create_tile_pool();
    println!("{:?}", tile_pool);
    println!("=================================================================================");
    let player_hand: Vec<[i32; 2]> = create_player_hand(&mut tile_pool);
    println!("{:?}", player_hand);
    println!("=================================================================================");
    let cpu_hand: Vec<[i32; 2]> = create_player_hand(&mut tile_pool);
    println!("{:?}", cpu_hand);
    println!("=================================================================================");
    println!("{:?}", tile_pool);
    println!("=================================================================================");
}
