use std::{io, print, println};

use crate::{
    can_play_tile::can_play_tile,
    place_tile::place_tile,
    types::{Hand, Tile},
};

pub fn player_turn(hand: &mut Hand, table: &mut Hand, edges: &Tile, pool: &mut Hand) {
    let mut tile_placed: bool = false;
    while !tile_placed {
        let can_draw = hand.iter().all(|tile| !can_play_tile(tile, edges));

        for (i, tile) in hand.iter().enumerate() {
            print!("{}:{:?}  ", i, tile);
        }

        if can_draw {
            println!("t:[take a tile]");
        } else {
            println!("Choose a playable tile.");
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la linea");
        let input = input.trim();
        println!("ingresaste: {}", input);

        match input {
            "t" => {
                if !can_draw {
                    println!("You may only draw when no playable tile exists.");
                } else if pool.is_empty() {
                    println!("No tiles left in the pool to draw.");
                    println!(
                        "Turn passes because there are no playable tiles and no tiles left to draw."
                    );
                    return;
                } else {
                    let drawn_tile = pool.pop().unwrap();
                    println!("You drew {:?}", drawn_tile);
                    hand.push(drawn_tile);
                }
            }
            _ => match input.parse::<usize>() {
                Ok(number) => {
                    if number < hand.len() {
                        if can_play_tile(&hand[number], edges) {
                            tile_placed = true;
                            let mut tile = hand.remove(number);
                            place_tile(&mut tile, edges, table);
                            println!("You played {:?}", tile);
                        } else {
                            println!(
                                "That tile cannot be placed on the board. Choose another tile."
                            );
                        }
                    } else {
                        println!(
                            "Index out of range. Choose a number from 0 to {}.",
                            hand.len().saturating_sub(1)
                        );
                    }
                }
                Err(_) => println!("Invalid input. Enter a tile index or 't' to draw."),
            },
        }
    }
}
