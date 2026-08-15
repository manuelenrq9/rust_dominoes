use crate::show_tiles::show_tiles;
use crate::types::Hand;

pub fn show_board(table: &Hand, player_hand: &Hand, cpu_hand: &Hand) {
    // Display current state at the start of each turn
    println!("\n================ TURN STATE ================");
    println!("Table:");
    show_tiles(&table);
    println!("Your hand:");
    show_tiles(&player_hand);
    println!("CPU hand (for debugging):");
    show_tiles(&cpu_hand);
}
