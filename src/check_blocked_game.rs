use crate::{
    copy_edges::copy_edges,
    decide_winner::decide_winner,
    player_can_play::player_can_play,
    types::{Hand, Tile},
};

pub fn check_blocked_game(
    player_hand: &Hand,
    cpu_hand: &Hand,
    tile_pool: &Hand,
    table: &Hand,
) -> bool {
    // Check for blocked game: pool empty and neither player can play
    let edges_now: Tile = copy_edges(&table);
    let can_player_play = player_can_play(&player_hand, &edges_now);
    let can_cpu_play = player_can_play(&cpu_hand, &edges_now);
    if tile_pool.is_empty() && !can_player_play && !can_cpu_play {
        println!(
            "No playable tiles for either player and the tile pool is empty. Game is blocked."
        );
        decide_winner(&player_hand, &cpu_hand);
        return true;
    }
    return false;
}
