use crate::{
    check_blocked_game::check_blocked_game,
    copy_edges::copy_edges,
    cpu_turn::cpu_turn,
    player_turn::player_turn,
    show_board::show_board,
    types::{Hand, Tile},
};

pub fn game_loop(
    table: &mut Hand,
    player_hand: &mut Hand,
    cpu_hand: &mut Hand,
    tile_pool: &mut Hand,
    player_turn_next: &mut bool,
) {
    // Game loop: alternate turns until one hand is empty
    loop {
        // Display current state at the start of each turn
        show_board(&table, &player_hand, &cpu_hand);

        // Check for blocked game: pool empty and neither player can play
        if check_blocked_game(&player_hand, &cpu_hand, &tile_pool, &table) {
            break;
        }

        if *player_turn_next {
            // Player turn
            println!("your turn!:");
            let edges_before = copy_edges(&table);
            player_turn(player_hand, table, &edges_before, tile_pool);
            if player_hand.is_empty() {
                println!("You win! Player ran out of tiles.");
                break;
            }
        } else {
            // CPU turn
            let edges_after_player: Tile = copy_edges(&table);
            println!("cpu's turn:");
            cpu_turn(cpu_hand, table, &edges_after_player, tile_pool);
            if cpu_hand.is_empty() {
                println!("CPU wins! CPU ran out of tiles.");
                break;
            }
        }

        *player_turn_next = !*player_turn_next;
    }
}
