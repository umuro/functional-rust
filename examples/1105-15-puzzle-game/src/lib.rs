//! 15-puzzle game
//!
//! Classic 4×4 sliding-tile puzzle: 15 numbered tiles and one empty space.
//! The goal is to slide tiles into ascending order (1–15, hole at bottom-right).
//!
//! ## Data model
//!
//! `board[pos]` = tile number at grid position `pos` (row-major, 0 = top-left).
//! The empty space is represented by tile number `0`.  The hole position is
//! cached in `hole` for O(1) lookup.
//!
//! This is the idiomatic Rust layout — the OCaml original uses the *inverse*
//! mapping (`positions[tile] = pos`), which is shown in `TileIndexed` below.

use std::fmt;

// ── Board-indexed representation (idiomatic Rust) ─────────────────────────────

/// 15-puzzle board — `board[position] = tile` (0 = empty space).
///
/// Positions are numbered row-major:
/// ```text
///  0  1  2  3
///  4  5  6  7
///  8  9 10 11
/// 12 13 14 15
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Puzzle {
    board: [u8; 16],
    hole: usize, // cached position of the empty space
}

impl Puzzle {
    /// Create a puzzle in the solved state (tiles 1–15 in order, hole at 15).
    pub fn new() -> Self {
        let mut board = [0u8; 16];
        for (i, tile) in board[..15].iter_mut().enumerate() {
            *tile = (i + 1) as u8;
        }
        Self { board, hole: 15 }
    }

    /// Position (0–15) of the empty space.
    pub fn hole_pos(&self) -> usize {
        self.hole
    }

    /// Tile number at a board position (`0` = empty).
    pub fn tile_at(&self, pos: usize) -> u8 {
        self.board[pos]
    }

    /// Tiles that are adjacent to the hole and can be slid into it.
    ///
    /// Returns tile numbers (1–15) in up/down/left/right order, skipping
    /// out-of-bounds neighbours.
    pub fn valid_moves(&self) -> Vec<u8> {
        self.adjacent_positions()
            .iter()
            .map(|&pos| self.board[pos])
            .collect()
    }

    /// Slide `tile` into the empty space.
    ///
    /// Returns `Err` if the tile is not adjacent to the hole.
    pub fn apply_move(&mut self, tile: u8) -> Result<(), &'static str> {
        let tile_pos = self
            .board
            .iter()
            .position(|&t| t == tile)
            .ok_or("tile not found")?;

        let (hr, hc) = (self.hole / 4, self.hole % 4);
        let (tr, tc) = (tile_pos / 4, tile_pos % 4);
        let adjacent = (hr == tr && hc.abs_diff(tc) == 1) || (hc == tc && hr.abs_diff(tr) == 1);

        if !adjacent {
            return Err("tile is not adjacent to the hole");
        }

        self.board.swap(self.hole, tile_pos);
        self.hole = tile_pos;
        Ok(())
    }

    /// Is the puzzle in the solved state?
    pub fn is_solved(&self) -> bool {
        self.hole == 15
            && self.board[..15]
                .iter()
                .enumerate()
                .all(|(i, &t)| t == (i + 1) as u8)
    }

    /// Shuffle by performing `n` random moves using a caller-supplied RNG.
    ///
    /// The closure must return a random `usize`; any range is fine — the value
    /// is reduced modulo the number of valid moves.  Accepting a closure makes
    /// the function deterministically testable without an external crate.
    ///
    /// Only moves to adjacent positions are made, so the puzzle always remains
    /// solvable.
    pub fn shuffle_with(&mut self, n: usize, mut rng: impl FnMut() -> usize) {
        for _ in 0..n {
            let adj = self.adjacent_positions();
            let pos = adj[rng() % adj.len()];
            self.board.swap(self.hole, pos);
            self.hole = pos;
        }
    }

    /// Render the board as a 4×4 grid string.
    ///
    /// Each cell is 4 characters wide; the empty space is shown as `"   ."`.
    pub fn display(&self) -> String {
        let mut out = String::with_capacity(5 * 4 * 4); // rough upper bound
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

    // ── Private helpers ──────────────────────────────────────────────────────

    /// Grid positions that are orthogonally adjacent to the hole.
    fn adjacent_positions(&self) -> Vec<usize> {
        let (r, c) = (self.hole / 4, self.hole % 4);
        [
            r.checked_sub(1).map(|r| r * 4 + c),              // up
            if r < 3 { Some((r + 1) * 4 + c) } else { None }, // down
            c.checked_sub(1).map(|c| r * 4 + c),              // left
            if c < 3 { Some(r * 4 + c + 1) } else { None },   // right
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

// ── Tile-indexed representation (mirrors OCaml) ───────────────────────────────

/// OCaml-mirror: `positions[tile] = board_position`.
///
/// This is the data layout used in the OCaml source — index by tile number,
/// store the board position.  `positions[0]` holds the hole's position.
///
/// Useful for showing the conceptual translation from OCaml to Rust.
#[derive(Clone, PartialEq, Eq)]
pub struct TileIndexed {
    positions: [u8; 16], // positions[tile] = board_position; [0] = hole_pos
}

impl TileIndexed {
    /// Create in solved state: tile i is at position i-1, hole at position 15.
    pub fn new() -> Self {
        let mut positions = [0u8; 16];
        positions[0] = 15; // hole at bottom-right
        for tile in 1u8..=15 {
            positions[tile as usize] = tile - 1; // tile i at position i-1
        }
        Self { positions }
    }

    /// Slide tile `n` into the hole — mirrors OCaml's `Puzzle.move p n`.
    pub fn move_tile(&mut self, n: usize) {
        let hole = self.positions[0];
        let tile_pos = self.positions[n];
        self.positions[0] = tile_pos;
        self.positions[n] = hole;
    }

    /// Is the puzzle solved?
    pub fn is_solved(&self) -> bool {
        self.positions[0] == 15 && (1u8..=15).all(|t| self.positions[t as usize] == t - 1)
    }
}

impl Default for TileIndexed {
    fn default() -> Self {
        Self::new()
    }
}

// ── Simple deterministic RNG (Lehmer LCG) for use in tests and examples ──────

/// Minimal LCG that produces a deterministic sequence from a seed.
///
/// Uses only `std` — no external crates needed.
pub struct Lcg(u64);

impl Lcg {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn sample(&mut self) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) as usize
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_solved() {
        assert!(Puzzle::new().is_solved());
    }

    #[test]
    fn test_hole_at_position_15_when_solved() {
        assert_eq!(Puzzle::new().hole_pos(), 15);
    }

    #[test]
    fn test_valid_moves_corner_hole() {
        // Solved state: hole at pos 15 (row 3, col 3) — only up and left.
        // tile at pos 11 = tile 12, tile at pos 14 = tile 15.
        let p = Puzzle::new();
        let mut moves = p.valid_moves();
        moves.sort_unstable();
        assert_eq!(moves, vec![12, 15]);
    }

    #[test]
    fn test_valid_moves_edge_hole() {
        // Move hole from 15 to 14 (apply tile 15).
        // Hole now at pos 14 (row 3, col 2): 3 neighbours — pos 10, 13, 15.
        let mut p = Puzzle::new();
        p.apply_move(15).unwrap();
        assert_eq!(p.hole_pos(), 14);
        let mut moves = p.valid_moves();
        moves.sort_unstable();
        // pos 10 = tile 11, pos 13 = tile 14, pos 15 now has tile 15
        assert_eq!(moves, vec![11, 14, 15]);
    }

    #[test]
    fn test_apply_move_updates_board_and_hole() {
        let mut p = Puzzle::new();
        // Tile 15 is at position 14, hole at 15.  Move tile 15 into hole.
        p.apply_move(15).unwrap();
        assert_eq!(p.hole_pos(), 14);
        assert_eq!(p.tile_at(14), 0);
        assert_eq!(p.tile_at(15), 15);
        assert!(!p.is_solved());
    }

    #[test]
    fn test_apply_move_invalid_tile_returns_error() {
        let mut p = Puzzle::new();
        // Tile 1 is at position 0, far from hole at 15 — not adjacent.
        assert!(p.apply_move(1).is_err());
        assert!(p.is_solved()); // board unchanged
    }

    #[test]
    fn test_one_shuffle_move_unsolved() {
        let mut p = Puzzle::new();
        // rng always returns 0 → picks first adjacent position.
        p.shuffle_with(1, || 0);
        assert!(!p.is_solved());
    }

    #[test]
    fn test_shuffle_20_moves_deterministic() {
        let mut p = Puzzle::new();
        let mut lcg = Lcg::new(42);
        p.shuffle_with(20, || lcg.sample());
        // After 20 moves with fixed seed the board must have changed.
        assert!(!p.is_solved());
    }

    #[test]
    fn test_display_solved_contains_all_tiles() {
        let d = Puzzle::new().display();
        for t in 1u8..=15 {
            assert!(d.contains(&t.to_string()), "tile {t} missing from display");
        }
        assert!(d.contains('.'), "hole marker missing");
    }

    #[test]
    fn test_tile_indexed_new_is_solved() {
        assert!(TileIndexed::new().is_solved());
    }

    #[test]
    fn test_tile_indexed_move_tile() {
        let mut p = TileIndexed::new();
        // Tile 15 is at position 14, hole at 15.  move_tile(15) swaps them.
        p.move_tile(15);
        assert_eq!(p.positions[0], 14); // hole moved to 14
        assert_eq!(p.positions[15], 15); // tile 15 at position 15
        assert!(!p.is_solved());
    }
}
