use crate::types::{Hand, Tile};

pub fn take_tile(hand: &mut Hand, index: usize) -> Tile {
    return hand.remove(index);
}
