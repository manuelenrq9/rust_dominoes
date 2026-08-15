use crate::types::{Hand, Tile};
use crate::{can_play_tile::can_play_tile, place_tile::place_tile};

pub fn cpu_turn(hand: &mut Hand, table: &mut Hand, edges: &Tile, pool: &mut Hand) {
    // Try to play the first playable tile from hand
    for i in 0..hand.len() {
        if can_play_tile(&hand[i], edges) {
            let mut tile = hand.remove(i);
            place_tile(&mut tile, edges, table);
            println!("CPU played {:?}", tile);
            return;
        }
    }

    // No playable tile in hand -> keep drawing until playable or pool empty
    loop {
        if pool.is_empty() {
            println!("CPU cannot play and the pool is empty. CPU passes.");
            return;
        }

        let mut drawn_tile = pool.pop().unwrap();
        println!("CPU drew {:?}", drawn_tile);
        if can_play_tile(&drawn_tile, edges) {
            place_tile(&mut drawn_tile, edges, table);
            println!("CPU played drawn tile {:?}", drawn_tile);
            return;
        } else {
            hand.push(drawn_tile);
            // continue drawing
        }
    }
}
