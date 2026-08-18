use crate::types::Hand;

pub fn decide_winner(player_hand: &Hand, cpu_hand: &Hand) {
    //The player with the lowest total pip count wins. If both players have the same total pip count, the game is a tie.
    let sum_pips = |h: &Hand| -> i32 { h.iter().map(|t| t[0] + t[1]).sum() };
    let p1 = sum_pips(&player_hand);
    let p2 = sum_pips(&cpu_hand);
    if p1 < p2 {
        println!("You win by lower pip total: {} vs {}.", p1, p2);
    } else if p2 < p1 {
        println!("CPU wins by lower pip total: {} vs {}.", p2, p1);
    } else {
        println!(
            "The game is a tie: equal tiles and pip totals ({} pips each).",
            p1
        );
    }
}
