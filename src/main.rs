mod create_tile_pool;

use crate::create_tile_pool::create_tile_pool;

fn main() {
    println!("Welcome to the Dominoes game, which is coded using the rust programming language!");
    let mut tile_pool: [[i32; 2]; 36] = create_tile_pool();
}
