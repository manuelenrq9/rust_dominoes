use std::{io, print, println};

use crate::{
    place_tile::place_tile,
    types::{Hand, Tile},
};

pub fn player_turn(hand: &mut Hand, table: &mut Hand, edges: &Tile, pool: &mut Hand) {
    let mut tile_placed: bool = false;
    while !tile_placed {
        for (i, tile) in hand.iter().enumerate() {
            print!("{}:{:?}  ", i, tile);
        }
        println!("t:[take a tile]");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la linea");
        let input = input.trim();
        println!("ingresaste: {}", input);

        match input {
            "t" => {
                if pool.is_empty() {
                    println!("No tiles left in the pool to draw.");
                } else {
                    let drawn_tile = pool.pop().unwrap();
                    println!("You drew {:?}", drawn_tile);
                    hand.push(drawn_tile);
                }
            }
            _ => match input.parse::<usize>() {
                Ok(number) => {
                    if number < hand.len() {
                        tile_placed = true;
                        place_tile(&mut hand.remove(number), edges, table);
                    } else {
                        println!(
                            "Index out of range. Choose a number from 0 to {}.",
                            hand.len().saturating_sub(1)
                        );
                    }
                }
                Err(e) => println!("Error parsing: {}", e),
            },
        }
    }
}
