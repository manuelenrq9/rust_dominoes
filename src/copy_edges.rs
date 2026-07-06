use crate::types::{Hand, Tile};

pub fn copy_edges(table: Hand) -> Tile {
    let first = match table.first() {
        Some(value) => *value,
        None => [0, 0],
    };
    let last = match table.last() {
        Some(value) => *value,
        None => [0, 0],
    };

    return [first[0], last[1]];
}
