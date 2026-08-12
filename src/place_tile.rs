use crate::types::{Hand, Tile};

pub fn place_tile(player_tile: &mut Tile, edges: &Tile, table: &mut Hand) {
    {
        'external: for (i, edges_value) in edges.iter().enumerate() {
            for (j, player_value) in player_tile.iter().enumerate() {
                if edges_value == player_value {
                    if i == j {
                        player_tile.swap(0, 1);
                    }
                    if i == 0 {
                        table.insert(0, *player_tile);
                    } else {
                        table.push(*player_tile);
                    }
                    break 'external;
                }
            }
        }
    }
}
