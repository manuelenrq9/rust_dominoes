use crate::types::Tile;

pub fn tile_is_double(tile: &Tile) -> bool {
    if tile[0] == tile[1] {
        return true;
    }
    return false;
}
