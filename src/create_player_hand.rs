use rand::Rng;

pub fn create_player_hand(pool: &mut Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    let mut player_hand: Vec<[i32; 2]> = Vec::new();
    let mut pool_len: usize;
    let mut rng = rand::rng();
    for _ in 0..=6 {
        pool_len = pool.len();
        let random_index = rng.random_range(0..pool_len);
        let random_value: [i32; 2] = pool.remove(random_index);
        player_hand.push(random_value);
    }
    return player_hand;
}
