use crate::decide_by_pips::decide_by_pips;
use crate::decide_by_tiles::decide_by_tiles;
use crate::types::{Hand, Tile};

pub fn decide_winner(player_hand: &Hand, cpu_hand: &Hand) {
    // Decide winner: fewest remaining tiles wins; if tied, lowest pip total wins; otherwise tie
    if !decide_by_tiles(&player_hand, &cpu_hand) {
        decide_by_pips(player_hand, cpu_hand);
    }
}
