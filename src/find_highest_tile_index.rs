use crate::get_tile_count::get_tile_count;

pub fn find_highest_tile_index(hand: &Vec<[i32; 2]>) -> usize {
    let mut highest_tile_count: i32 = 0;
    let mut tile_count: i32;
    let mut current_tile_index: usize = 0;

    for (i, tile) in hand.iter().enumerate() {
        tile_count = get_tile_count(&tile);
        if tile_count > highest_tile_count {
            highest_tile_count = tile_count;
            current_tile_index = i;
        }
    }

    return current_tile_index;
}
