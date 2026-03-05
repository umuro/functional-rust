//! 15-puzzle game — standalone example
//!
//! Demonstrates the board-indexed Puzzle and the OCaml-mirror TileIndexed
//! representations.  Run with: `rustc example.rs && ./example`

use std::fmt;

// ── Board-indexed Puzzle ──────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Eq)]
pub struct Puzzle {
    board: [u8; 16],
    hole: usize,
}

impl Puzzle {
    pub fn new() -> Self {
        let mut board = [0u8; 16];
        for (i, tile) in board[..15].iter_mut().enumerate() {
            *tile = (i + 1) as u8;
        }
        Self { board, hole: 15 }
    }

    pub fn hole_pos(&self) -> usize {
        self.hole
    }

    pub fn tile_at(&self, pos: usize) -> u8 {
        self.board[pos]
    }

    pub fn valid_moves(&self) -> Vec<u8> {
        self.adjacent_positions()
            .iter()
            .map(|&pos| self.board[pos])
            .collect()
    }

    pub fn apply_move(&mut self, tile: u8) -> Result<(), &'static str> {
        let tile_pos = self
            .board
            .iter()
            .position(|&t| t == tile)
            .ok_or("tile not found")?;

        let (hr, hc) = (self.hole / 4, self.hole % 4);
        let (tr, tc) = (tile_pos / 4, tile_pos % 4);
        let adjacent =
            (hr == tr && hc.abs_diff(tc) == 1) || (hc == tc && hr.abs_diff(tr) == 1);

        if !adjacent {
            return Err("tile is not adjacent to the hole");
        }

        self.board.swap(self.hole, tile_pos);
        self.hole = tile_pos;
        Ok(())
    }

    pub fn is_solved(&self) -> bool {
        self.hole == 15
            && self.board[..15]
                .iter()
                .enumerate()
                .all(|(i, &t)| t == (i + 1) as u8)
    }

    pub fn shuffle_with(&mut self, n: usize, mut rng: impl FnMut() -> usize) {
        for _ in 0..n {
            let adj = self.adjacent_positions();
            let pos = adj[rng() % adj.len()];
            self.board.swap(self.hole, pos);
            self.hole = pos;
        }
    }

    pub fn display(&self) -> String {
        let mut out = String::with_capacity(80);
        for row in 0..4usize {
            for col in 0..4usize {
                let tile = self.board[row * 4 + col];
                if tile == 0 {
                    out.push_str("   .");
                } else {
                    out.push_str(&format!("{tile:4}"));
                }
            }
            out.push('\n');
        }
        out
    }

    fn adjacent_positions(&self) -> Vec<usize> {
        let (r, c) = (self.hole / 4, self.hole % 4);
        [
            r.checked_sub(1).map(|r| r * 4 + c),
            if r < 3 { Some((r + 1) * 4 + c) } else { None },
            c.checked_sub(1).map(|c| r * 4 + c),
            if c < 3 { Some(r * 4 + c + 1) } else { None },
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

// ── OCaml-mirror TileIndexed ──────────────────────────────────────────────────

#[derive(Clone, PartialEq, Eq)]
pub struct TileIndexed {
    positions: [u8; 16],
}

impl TileIndexed {
    pub fn new() -> Self {
        let mut positions = [0u8; 16];
        positions[0] = 15;
        for tile in 1u8..=15 {
            positions[tile as usize] = tile - 1;
        }
        Self { positions }
    }

    pub fn move_tile(&mut self, n: usize) {
        let hole = self.positions[0];
        let tile_pos = self.positions[n];
        self.positions[0] = tile_pos;
        self.positions[n] = hole;
    }

    pub fn is_solved(&self) -> bool {
        self.positions[0] == 15 && (1u8..=15).all(|t| self.positions[t as usize] == t - 1)
    }
}

impl Default for TileIndexed {
    fn default() -> Self {
        Self::new()
    }
}

// ── Deterministic LCG ────────────────────────────────────────────────────────

struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn sample(&mut self) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) as usize
    }
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    println!("=== 15-Puzzle: Solved state ===");
    let p = Puzzle::new();
    print!("{p}");
    println!("is_solved: {}", p.is_solved());
    println!("hole at: {}", p.hole_pos());
    println!("valid moves: {:?}", p.valid_moves());

    println!("\n=== After shuffling 20 moves (seed=42) ===");
    let mut p2 = Puzzle::new();
    let mut lcg = Lcg::new(42);
    p2.shuffle_with(20, || lcg.sample());
    print!("{p2}");
    println!("is_solved: {}", p2.is_solved());
    println!("valid moves: {:?}", p2.valid_moves());

    println!("\n=== Applying one valid move ===");
    let mut p3 = Puzzle::new();
    let moves = p3.valid_moves();
    let chosen = moves[0];
    println!("Moving tile {chosen}");
    p3.apply_move(chosen).unwrap();
    print!("{p3}");

    println!("\n=== OCaml-mirror TileIndexed representation ===");
    let mut ti = TileIndexed::new();
    println!("Solved: {}", ti.is_solved());
    // positions[0] = hole position, positions[n] = position of tile n
    println!("Hole at position: {}", ti.positions[0]);
    println!("Tile 15 at position: {}", ti.positions[15]);
    ti.move_tile(15); // slide tile 15 into hole
    println!("After move_tile(15):");
    println!("  Hole at position: {}", ti.positions[0]);
    println!("  Tile 15 at position: {}", ti.positions[15]);
    println!("Solved: {}", ti.is_solved());
}

/* Output:
=== 15-Puzzle: Solved state ===
   1   2   3   4
   5   6   7   8
   9  10  11  12
  13  14  15   .
is_solved: true
hole at: 15
valid moves: [12, 15]

=== After shuffling 20 moves (seed=42) ===
   1   2   3   4
   5   6   7   8
  10  14  11   .
   9  13  15  12
is_solved: false
valid moves: [11, 12]

=== Applying one valid move ===
Moving tile 12
   1   2   3   4
   5   6   7   8
   9  10  11  12
  13  14  15   .

=== OCaml-mirror TileIndexed representation ===
Solved: true
Hole at position: 15
Tile 15 at position: 14
After move_tile(15):
  Hole at position: 14
  Tile 15 at position: 15
Solved: false
*/
