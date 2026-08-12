use crate::types::Tile;

pub fn can_play_tile(tile: &Tile, edges: &Tile) -> bool {
    for edge_value in edges.iter() {
        for tile_value in tile.iter() {
            if edge_value == tile_value {
                return true;
            }
        }
    }
    false
}
