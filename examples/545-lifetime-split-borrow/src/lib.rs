//! Split Borrows from Structs
//!
//! Borrowing different fields simultaneously.

pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    pub enemies: Vec<(f32, f32)>,
    pub score: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player_x: 0.0,
            player_y: 0.0,
            enemies: Vec::new(),
            score: 0,
        }
    }

    /// Split borrow: &mut player position, &enemies
    pub fn get_refs(&mut self) -> (&mut f32, &mut f32, &[(f32, f32)]) {
        (&mut self.player_x, &mut self.player_y, &self.enemies)
    }

    /// Borrow different fields independently
    pub fn update_position(&mut self, dx: f32, dy: f32) {
        self.player_x += dx;
        self.player_y += dy;
    }

    pub fn add_enemy(&mut self, x: f32, y: f32) {
        self.enemies.push((x, y));
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

/// Demonstrate split borrowing.
pub fn split_borrow_demo(state: &mut GameState) {
    let (px, py, enemies) = state.get_refs();
    *px = 10.0;
    *py = 20.0;
    for (ex, ey) in enemies {
        println!("Enemy at ({}, {})", ex, ey);
    }
}

/// Two-field struct for simpler example.
pub struct Pair {
    pub left: String,
    pub right: String,
}

impl Pair {
    pub fn get_both(&mut self) -> (&mut String, &mut String) {
        (&mut self.left, &mut self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_borrow() {
        let mut state = GameState::new();
        state.enemies.push((1.0, 2.0));

        let (px, py, enemies) = state.get_refs();
        *px = 5.0;
        *py = 10.0;
        assert_eq!(enemies.len(), 1);
    }

    #[test]
    fn test_pair_split() {
        let mut pair = Pair {
            left: String::from("L"),
            right: String::from("R"),
        };
        let (l, r) = pair.get_both();
        l.push_str("eft");
        r.push_str("ight");
        assert_eq!(pair.left, "Left");
        assert_eq!(pair.right, "Right");
    }

    #[test]
    fn test_sequential_borrows() {
        let mut state = GameState::new();
        state.update_position(1.0, 1.0);
        state.add_enemy(5.0, 5.0);
        assert_eq!(state.player_x, 1.0);
        assert_eq!(state.enemies.len(), 1);
    }
}
