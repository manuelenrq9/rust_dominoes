mod can_play_tile;
mod copy_edges;
mod cpu_turn;
mod create_player_hand;
mod create_tile_pool;
mod find_highest_tile_index;
mod find_starter_tile;
mod get_tile_count;
mod place_tile;
mod player_turn;
mod show_board;
mod show_tiles;
mod take_tile;
mod tile_is_double;
mod types;
use crate::{
    copy_edges::copy_edges,
    cpu_turn::cpu_turn,
    create_player_hand::create_player_hand,
    create_tile_pool::create_tile_pool,
    find_starter_tile::find_starter_tile,
    player_turn::player_turn,
    show_board::show_board,
    show_tiles::show_tiles,
    types::{Hand, Tile},
};

fn main() {
    println!("Welcome to the Dominoes game!");
    let mut tile_pool: Hand = create_tile_pool();
    let mut player_hand: Hand = create_player_hand(&mut tile_pool);
    let mut cpu_hand: Hand = create_player_hand(&mut tile_pool);
    let (starter_tile, starter_was_player) = find_starter_tile(&mut player_hand, &mut cpu_hand);
    let mut table: Hand = vec![starter_tile];
    println!("Starter tile placed on the table.");
    println!("Your hand:");
    show_tiles(&player_hand);
    println!("CPU hand (hidden during final game):");
    show_tiles(&cpu_hand);
    println!("Initial table:");
    show_tiles(&table);
    // Determine who plays next: the player who DID NOT have the starter plays next
    let mut player_turn_next: bool = !starter_was_player;

    // Game loop: alternate turns until one hand is empty
    loop {
        // Display current state at the start of each turn
        show_board(&table, &player_hand, &cpu_hand);

        if player_turn_next {
            // Player turn
            println!("your turn!:");
            let edges_before = copy_edges(&table);
            player_turn(&mut player_hand, &mut table, &edges_before, &mut tile_pool);
            if player_hand.is_empty() {
                println!("You win! Player ran out of tiles.");
                break;
            }
        } else {
            // CPU turn
            let edges_after_player: Tile = copy_edges(&table);
            println!("cpu's turn:");
            cpu_turn(
                &mut cpu_hand,
                &mut table,
                &edges_after_player,
                &mut tile_pool,
            );
            if cpu_hand.is_empty() {
                println!("CPU wins! CPU ran out of tiles.");
                break;
            }
        }

        player_turn_next = !player_turn_next;
    }
}
