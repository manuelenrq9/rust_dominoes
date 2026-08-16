use crate::{
    can_play_tile::can_play_tile,
    types::{Hand, Tile},
};

pub fn player_can_play(player_hand: &Hand, edges: &Tile) -> bool {
    for tile in player_hand.iter() {
        if can_play_tile(tile, edges) {
            return true;
        }
    }
    return false;
}
