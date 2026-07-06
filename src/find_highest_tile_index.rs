use crate::{get_tile_count::get_tile_count, tile_is_double::tile_is_double, types::Hand};

pub fn find_highest_tile_index(hand: &Hand) -> usize {
    let mut highest_tile_count: i32 = 0;
    let mut tile_count: i32;
    let mut current_tile_index: usize = 0;
    let mut double_has_been_found: bool = false;

    for (i, tile) in hand.iter().enumerate() {
        if tile_is_double(tile) {
            if !double_has_been_found {
                double_has_been_found = true;
                current_tile_index = i;
            }

            tile_count = get_tile_count(&tile);
            if tile_count > highest_tile_count {
                highest_tile_count = tile_count;
                current_tile_index = i;
            }
        }
    }
    if !double_has_been_found {
        for (i, tile) in hand.iter().enumerate() {
            tile_count = get_tile_count(&tile);
            if tile_count > highest_tile_count {
                highest_tile_count = tile_count;
                current_tile_index = i;
            }
        }
    }

    return current_tile_index;
}
