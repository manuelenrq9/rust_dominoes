pub fn tile_is_double(tile: &[i32; 2]) -> bool {
    if tile[0] == tile[1] {
        return true;
    }
    return false;
}
