# OCaml vs Rust: 15-Puzzle Game

## Side-by-Side Code

### OCaml
```ocaml
module Puzzle = struct
  type t = int array

  let make () =
    [| 15;  (* index 0 = hole position *)
        0;  1;  2;  3;
        4;  5;  6;  7;
        8;  9; 10; 11;
       12; 13; 14;  |]

  let move p n =
    let hole, i = p.(0), p.(n) in
    p.(0) <- i;
    p.(n) <- hole

  let shuffle p n =
    for _i = 1 to n do
      move p (1 + Random.int 15)
    done
end
```

### Rust — idiomatic (board-indexed)
```rust
#[derive(Clone, PartialEq, Eq)]
pub struct Puzzle {
    board: [u8; 16],   // board[position] = tile (0 = hole)
    hole: usize,       // cached position of the empty space
}

impl Puzzle {
    pub fn apply_move(&mut self, tile: u8) -> Result<(), &'static str> {
        let tile_pos = self.board.iter().position(|&t| t == tile)
            .ok_or("tile not found")?;
        let (hr, hc) = (self.hole / 4, self.hole % 4);
        let (tr, tc) = (tile_pos / 4, tile_pos % 4);
        let adjacent =
            (hr == tr && hc.abs_diff(tc) == 1) ||
            (hc == tc && hr.abs_diff(tr) == 1);
        if !adjacent {
            return Err("tile is not adjacent to the hole");
        }
        self.board.swap(self.hole, tile_pos);
        self.hole = tile_pos;
        Ok(())
    }

    pub fn shuffle_with(&mut self, n: usize, mut rng: impl FnMut() -> usize) {
        for _ in 0..n {
            let adj = self.adjacent_positions();
            let pos = adj[rng() % adj.len()];
            self.board.swap(self.hole, pos);
            self.hole = pos;
        }
    }
}
```

### Rust — functional/OCaml-mirror (tile-indexed)
```rust
/// positions[tile] = board_position;  positions[0] = hole_pos
pub struct TileIndexed {
    positions: [u8; 16],
}

impl TileIndexed {
    pub fn move_tile(&mut self, n: usize) {
        let hole = self.positions[0];
        let tile_pos = self.positions[n];
        self.positions[0] = tile_pos;
        self.positions[n] = hole;
    }

    pub fn is_solved(&self) -> bool {
        self.positions[0] == 15
            && (1u8..=15).all(|t| self.positions[t as usize] == t - 1)
    }
}
```

## Type Signatures

| Concept | OCaml | Rust (idiomatic) |
|---------|-------|-----------------|
| Board type | `int array` (17 ints) | `[u8; 16]` + `usize` hole |
| Array index direction | tile → position | position → tile |
| Move result | unit (silent) | `Result<(), &'static str>` |
| Shuffle RNG | `Random.int` (global) | `impl FnMut() -> usize` (injected) |
| Output | `print_string` (stdout) | `String` + `impl Display` |

## Key Insights

1. **Inverse representations:** The OCaml source uses a tile-indexed array
   (`p.(tile) = position`), treating the hole as tile zero.  Idiomatic Rust
   flips it to a board-indexed array (`board[position] = tile`), which makes
   the `Display` implementation a simple row-major scan — no scatter step
   needed.  Both carry identical information; the choice affects which
   direction is O(1) lookup.

2. **Adjacency validation:** OCaml's `move` unconditionally swaps any two
   positions, so passing a bad tile number silently corrupts state.  Rust's
   `apply_move` uses `abs_diff` and a checked row/column comparison to verify
   adjacency before touching the board, returning `Err` on bad input.  This
   keeps the puzzle invariant intact without panicking.

3. **Testable randomness via closure injection:** The OCaml shuffle calls
   `Random.int` directly, making it impossible to test deterministically
   without mocking the global RNG.  Rust's `shuffle_with` accepts
   `impl FnMut() -> usize`, so tests pass `|| 0` or a seeded LCG closure
   without any external crate — pure `std`.

4. **I/O separation:** The OCaml `print` function writes directly to stdout,
   mixing logic and output.  Rust implements `Display` on `Puzzle` (and
   exposes `display() -> String`), so the rendering is inspectable in tests
   with `assert!(d.contains('.'))` and the interactive loop can be added in
   `main` without touching the library.

5. **Ownership and caching:** OCaml arrays are always mutable references;
   Rust caches `hole: usize` alongside `board` so adjacency checks never scan
   the board — the hole position is always known in O(1).  The borrow checker
   ensures `hole` stays consistent with `board` because both live in the same
   `&mut self` receiver.

## When to Use Each Style

**Use board-indexed Rust (idiomatic) when:** you need fast display, frequent
"what's at position X?" queries, or want simple `Display` without a scatter
pass.

**Use tile-indexed Rust (OCaml-mirror) when:** you need fast "where is tile
N?" lookups — e.g., solving algorithms that track each tile's current
position to compute a heuristic like Manhattan distance.
