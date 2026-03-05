//! # 545. Split Borrows from Structs
//! Borrowing different fields simultaneously.

struct GameState {
    player_x: f32,
    player_y: f32,
    enemies: Vec<(f32, f32)>,
    score: u32,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player_x: 0.0,
            player_y: 0.0,
            enemies: vec![(5.0, 5.0), (10.0, 3.0)],
            score: 0,
        }
    }

    /// Split borrows via direct field access in methods
    fn update_and_score(&mut self) {
        // Direct field access — borrow checker sees separate fields
        let px = self.player_x;
        let py = self.player_y;

        // Borrow enemies (immutable) while reading player position
        let close_enemies: Vec<_> = self.enemies.iter()
            .filter(|&&(ex, ey)| {
                let dx = ex - px;
                let dy = ey - py;
                (dx * dx + dy * dy).sqrt() < 10.0
            })
            .collect();

        // Mutate score (different field from enemies)
        self.score += close_enemies.len() as u32 * 10;

        println!("Close enemies: {}, score: {}", close_enemies.len(), self.score);
    }

    /// Return refs to separate fields for caller to use simultaneously
    fn player_and_score_mut(&mut self) -> (&mut f32, &mut f32, &mut u32) {
        (&mut self.player_x, &mut self.player_y, &mut self.score)
    }
}

/// Struct that makes split borrows explicit
struct SplitStruct {
    header: Vec<u8>,
    body: Vec<u8>,
    footer: Vec<u8>,
}

impl SplitStruct {
    fn header_and_body_mut(&mut self) -> (&mut Vec<u8>, &mut Vec<u8>) {
        (&mut self.header, &mut self.body)
    }

    fn all_three_mut(&mut self) -> (&mut Vec<u8>, &mut Vec<u8>, &mut Vec<u8>) {
        (&mut self.header, &mut self.body, &mut self.footer)
    }
}

fn main() {
    // Direct field access — split borrows work
    let mut gs = GameState::new();
    gs.update_and_score();

    // Split mutable borrows via method
    let (px, py, score) = gs.player_and_score_mut();
    *px = 3.0;
    *py = 4.0;
    *score += 100;
    println!("After split update: player=({:.1},{:.1}), score={}", gs.player_x, gs.player_y, gs.score);

    // SplitStruct
    let mut ss = SplitStruct {
        header: b"HEADER".to_vec(),
        body: b"BODY".to_vec(),
        footer: b"FOOTER".to_vec(),
    };

    let (h, b) = ss.header_and_body_mut();
    h.push(b'!');
    b.push(b'?');
    println!("header: {:?}", String::from_utf8_lossy(&ss.header));
    println!("body: {:?}", String::from_utf8_lossy(&ss.body));

    // Vec split — classic split borrow pattern
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = v.split_at_mut(3);
    left[0] += 100;
    right[0] += 200;
    println!("split v: {:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_mutable() {
        let mut gs = GameState::new();
        let (px, py, score) = gs.player_and_score_mut();
        *px = 5.0; *py = 12.0; *score = 42;
        assert_eq!(gs.player_x, 5.0);
        assert_eq!(gs.score, 42);
    }

    #[test]
    fn test_vec_split_at_mut() {
        let mut v = vec![0, 1, 2, 3, 4];
        let (l, r) = v.split_at_mut(2);
        l[0] = 10;
        r[0] = 20;
        assert_eq!(v, [10, 1, 20, 3, 4]);
    }
}
