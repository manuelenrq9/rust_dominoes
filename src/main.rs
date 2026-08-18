mod can_play_tile;
mod check_blocked_game;
mod copy_edges;
mod cpu_turn;
mod create_player_hand;
mod create_tile_pool;
mod decide_winner;
mod find_highest_tile_index;
mod find_starter_tile;
mod game_loop;
mod get_tile_count;
mod place_tile;
mod player_can_play;
mod player_turn;
mod show_board;
mod show_tiles;
mod take_tile;
mod tile_is_double;
mod types;
use crate::{
    create_player_hand::create_player_hand, create_tile_pool::create_tile_pool,
    find_starter_tile::find_starter_tile, game_loop::game_loop, show_tiles::show_tiles,
    types::Hand,
};

fn main() {
    println!("Welcome to the Dominoes game!");
    let mut tile_pool: Hand = create_tile_pool();
    let mut player_hand: Hand = create_player_hand(&mut tile_pool);
    let mut cpu_hand: Hand = create_player_hand(&mut tile_pool);
    let (starter_tile, starter_was_player) = find_starter_tile(&mut player_hand, &mut cpu_hand);
    let mut table: Hand = vec![starter_tile];
    println!("Starter tile placed on the table.");
    println!("CPU hand:");
    show_tiles(&cpu_hand);
    println!("Your hand:");
    show_tiles(&player_hand);
    println!("Initial table:");
    show_tiles(&table);

    // Determine who plays next: the player who DID NOT have the starter plays next
    let mut player_turn_next: bool = !starter_was_player;

    // Game loop: alternate turns until one hand is empty
    game_loop(
        &mut table,
        &mut player_hand,
        &mut cpu_hand,
        &mut tile_pool,
        &mut player_turn_next,
    );
}
