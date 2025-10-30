use rand::Rng;

pub fn create_player_hand(pool: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut player_hand: Vec<Vec<i32>> = Vec::new();
    let mut pool_len: usize;
    let mut rng = rand::rng();
    for _ in 0..=6 {
        pool_len = pool.len();
        let random_index = rng.random_range(0..pool_len);
        let mut random_value: Vec<i32> = Vec::new();
        random_value.push(pool[random_index][0]);
        random_value.push(pool[random_index][1]);
        player_hand.push(random_value);
    }
    for i in 0..=6 {
        println!(
            "[{},{}]",
            player_hand[i as usize][0], player_hand[i as usize][1]
        );
    }
    return player_hand;
}
