rust_dominoes
===============

A small terminal Dominoes game written in Rust for learning and experimentation.

Overview
--------
- Two players: human vs CPU.
- Each player has a hand of tiles (pairs of numbers).
- A starter tile is chosen from the hands and placed on the table.
- The player who did *not* provide the starter tile goes first.
- On a turn a player must place a playable tile (one end matches a table edge). If no playable tile exists the player draws from the pool until they find a playable tile or the pool is empty. If the pool is empty and they still cannot play, the turn passes.
- The first player to run out of tiles wins.

Controls
--------
- On your turn the program prints three sections: the table, your hand, and the CPU hand (visible for debugging).
- To play a tile enter its index (the number printed to the left of each tile).
- If you have no playable tiles, type `t` to draw from the pool. Drawing repeats automatically until you draw a playable tile or the pool is empty.

Build & Run
-----------
Requirements: Rust toolchain (rustc and cargo).

From the project root:

```bash
cargo build
cargo run
```

Files of interest
-----------------
- `src/main.rs` — program entry, game loop and turn sequencing.
- `src/player_turn.rs` — human player turn logic and input handling.
- `src/cpu_turn.rs` — CPU turn logic.
- `src/place_tile.rs` — places a tile on the table when it matches an edge.
- `src/can_play_tile.rs` — helper that checks whether a tile can be played on current edges.
- `src/show_tiles.rs` — prints a concise, indexed view of a hand.
- `src/types.rs` — basic alias types: `Tile` and `Hand`.

Development notes
-----------------
- The CPU behaviour is intentionally simple (plays the first playable tile it finds).
- The CPU hand is printed for debugging; you can hide it in `main.rs` when you want a final UX.

Contributing
------------
Feel free to open issues or submit changes. The code is intentionally small and educational.

License
-------
Unlicensed — use freely for learning and experimentation.
