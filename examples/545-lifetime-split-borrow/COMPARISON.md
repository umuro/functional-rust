# OCaml vs Rust: Split Borrows

## OCaml
```ocaml
(* No concept of split borrows — records are immutable or use refs *)
type game_state = {
  mutable player_x: float;
  mutable player_y: float;
  enemies: (float * float) list ref;
}

(* Can mutate any field anytime *)
```

## Rust
```rust
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    pub enemies: Vec<(f32, f32)>,
}

impl GameState {
    // Split borrow: different fields simultaneously
    pub fn get_refs(&mut self) -> (&mut f32, &mut f32, &[(f32, f32)]) {
        (&mut self.player_x, &mut self.player_y, &self.enemies)
    }
}
```

## Key Differences

1. **OCaml**: Mutable fields independent, no tracking
2. **Rust**: Compiler tracks field-level borrows
3. **Rust**: Can borrow different fields simultaneously
4. **Rust**: Prevents aliasing of same field
5. Both: Enable efficient in-place updates
