pub fn get_tile_count(tile: &[i32; 2]) -> i32 {
    let mut count: i32 = 0;
    for value in tile {
        count += value;
    }
    return count;
}
