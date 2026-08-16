use crate::types::{Hand, Tile};

pub fn decide_by_tiles(player_hand: &Hand, cpu_hand: &Hand) -> bool {
    // Decide winner: fewest remaining tiles wins; if tied, lowest pip total wins; otherwise tie
    if player_hand.len() < cpu_hand.len() {
        println!(
            "You win by fewer tiles: {} vs {}.",
            player_hand.len(),
            cpu_hand.len()
        );
        return true;
    } else if cpu_hand.len() < player_hand.len() {
        println!(
            "CPU wins by fewer tiles: {} vs {}.",
            cpu_hand.len(),
            player_hand.len()
        );
        return true;
    } else {
        return false;
    }
}
