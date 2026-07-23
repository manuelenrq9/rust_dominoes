use std::{io, print, println};

use crate::types::Hand;

pub fn player_turn(hand: &mut Hand) {
    for (i, tile) in hand.iter().enumerate() {
        print!("{}:{:?}  ", i, tile);
    }
    print!("t:[take a tile]");
    println!();

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la linea");
    let entrada = entrada.trim();
    println!("ingresaste: {}", entrada)
}
