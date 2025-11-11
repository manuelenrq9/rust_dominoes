use crate::{
    find_highest_tile_index::find_highest_tile_index, get_tile_count::get_tile_count,
    tile_is_double::tile_is_double,
};

pub fn find_starter_tile(hand1: &mut Vec<[i32; 2]>, hand2: &mut Vec<[i32; 2]>) -> [i32; 2] {
    let index1: usize = find_highest_tile_index(&hand1);
    let index2: usize = find_highest_tile_index(&hand2);
    let starter_tile: [i32; 2];

    if (tile_is_double(&hand1[index1]) && tile_is_double(&hand2[index2]))
        || (!tile_is_double(&hand1[index1]) && !tile_is_double(&hand2[index2]))
    {
        if get_tile_count(&hand1[index1]) > get_tile_count(&hand2[index2]) {
            starter_tile = hand1.remove(index1);
        } else {
            starter_tile = hand2.remove(index2);
        }
    } else if tile_is_double(&hand1[index1]) {
        starter_tile = hand1.remove(index1);
    } else {
        starter_tile = hand2.remove(index2);
    }

    return starter_tile;
}
