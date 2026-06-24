mod create_player_hand;
mod create_tile_pool;
mod find_highest_tile_index;
mod find_starter_tile;
mod get_tile_count;
mod show_tiles;
mod tile_is_double;

use crate::{
    create_player_hand::create_player_hand, create_tile_pool::create_tile_pool,
    find_starter_tile::find_starter_tile, show_tiles::show_tiles,
};

fn main() {
    println!("Welcome to the Dominoes game!");
    let mut tile_pool: Vec<[i32; 2]> = create_tile_pool();
    println!("tile pool: ");
    show_tiles(&tile_pool);
    println!("=================================================================================");
    let mut player_hand: Vec<[i32; 2]> = create_player_hand(&mut tile_pool);
    println!("player hand: ");
    show_tiles(&player_hand);
    println!("=================================================================================");
    let mut cpu_hand: Vec<[i32; 2]> = create_player_hand(&mut tile_pool);
    println!("cpu hand: ");
    show_tiles(&cpu_hand);
    println!("=================================================================================");
    println!("tile pool: ");
    show_tiles(&tile_pool);
    println!("=================================================================================");
    println!("=================================================================================");
    println!("=================================================================================");
    let table: Vec<[i32; 2]> = vec![find_starter_tile(&mut player_hand, &mut cpu_hand)];
    println!("table: ");
    show_tiles(&table);
    println!("=================================================================================");
    println!("player hand: ");
    show_tiles(&player_hand);
    println!("=================================================================================");
    println!("cpu hand: ");
    show_tiles(&cpu_hand);
}
