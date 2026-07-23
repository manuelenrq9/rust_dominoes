mod copy_edges;
mod create_player_hand;
mod create_tile_pool;
mod find_highest_tile_index;
mod find_starter_tile;
mod get_tile_count;
mod player_turn;
mod show_tiles;
mod take_tile;
mod tile_is_double;
mod types;
use crate::{
    copy_edges::copy_edges,
    create_player_hand::create_player_hand,
    create_tile_pool::create_tile_pool,
    find_starter_tile::find_starter_tile,
    player_turn::player_turn,
    show_tiles::show_tiles,
    types::{Hand, Tile},
};

fn main() {
    println!("Welcome to the Dominoes game!");
    let mut tile_pool: Hand = create_tile_pool();
    println!("tile pool: ");
    show_tiles(&tile_pool);
    println!("=================================================================================");
    let mut player_hand: Hand = create_player_hand(&mut tile_pool);
    println!("player hand: ");
    show_tiles(&player_hand);
    println!("=================================================================================");
    let mut cpu_hand: Hand = create_player_hand(&mut tile_pool);
    println!("cpu hand: ");
    show_tiles(&cpu_hand);
    println!("=================================================================================");
    println!("tile pool: ");
    show_tiles(&tile_pool);
    println!("edges: ");
    let edges1: Tile = copy_edges(tile_pool);
    println!("{:?}", edges1);
    println!("=================================================================================");
    println!("=================================================================================");
    println!("=================================================================================");
    let table: Hand = vec![find_starter_tile(&mut player_hand, &mut cpu_hand)];
    println!("table: ");
    show_tiles(&table);
    println!("edges: ");
    let edges2: Tile = copy_edges(table);
    println!("{:?}", edges2);
    println!("=================================================================================");
    println!("player hand: ");
    show_tiles(&player_hand);
    println!("=================================================================================");
    println!("cpu hand: ");
    show_tiles(&cpu_hand);
    println!("your turn!:");
    player_turn(&mut player_hand);
}
