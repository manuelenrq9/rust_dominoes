use crate::{
    find_highest_tile_index::find_highest_tile_index,
    get_tile_count::get_tile_count,
    take_tile::take_tile,
    tile_is_double::tile_is_double,
    types::{Hand, Tile},
};

pub fn find_starter_tile(hand1: &mut Hand, hand2: &mut Hand) -> Tile {
    let index1: usize = find_highest_tile_index(&hand1);
    let index2: usize = find_highest_tile_index(&hand2);
    let tile1: &Tile = &hand1[index1];
    let tile2: &Tile = &hand2[index2];
    let starter_tile: [i32; 2];

    if (tile_is_double(tile1) && tile_is_double(tile2))
        || (!tile_is_double(tile1) && !tile_is_double(tile2))
    {
        if get_tile_count(tile1) > get_tile_count(tile2) {
            starter_tile = take_tile(hand1, index1);
        } else {
            starter_tile = take_tile(hand2, index2);
        }
    } else if tile_is_double(&hand1[index1]) {
        starter_tile = take_tile(hand1, index1);
    } else {
        starter_tile = take_tile(hand2, index2);
    }

    return starter_tile;
}
