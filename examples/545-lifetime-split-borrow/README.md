📖 **[View on hightechmind.io →](https://hightechmind.io/rust/545-lifetime-split-borrow)**

---

# Split Borrows from Structs
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A common pattern in game engines and simulations: update the player's position while reading the enemy list from the same `GameState`. Naively, borrowing `&mut self` prevents reading any field — the entire struct is "mutably borrowed." But Rust's borrow checker actually tracks borrows at the field level, not just the struct level. Split borrows let you hold `&mut` to one field and `&` to another simultaneously, as long as they do not alias. This is critical for performance-sensitive code that avoids unnecessary cloning.

## Learning Outcomes

- How Rust's borrow checker tracks borrows at the field level for plain structs
- How `get_refs(&mut self) -> (&mut f32, &mut f32, &[(f32, f32)])` enables simultaneous field borrows
- Why split borrows work for structs but not through a method call on `self` in general
- How `split_at_mut` and `split_first_mut` provide split borrows on slices
- Where split borrows matter: ECS systems, game state, embedded systems with shared hardware registers

## Rust Application

`GameState` holds player coordinates and an enemy list. `get_refs(&mut self)` returns a tuple of `&mut f32`, `&mut f32`, and `&[(f32, f32)]` — three simultaneous borrows of different fields. Rust accepts this because `player_x`, `player_y`, and `enemies` are distinct fields and cannot alias. For slices, `split_at_mut(mid)` returns `(&mut [T], &mut [T])` — two non-overlapping mutable slice halves, which is safe precisely because they partition the original slice.

Key patterns:
- `(&mut self.field_a, &mut self.field_b, &self.field_c)` — direct field split
- `slice.split_at_mut(mid)` — standard library split borrow for slices
- `slice.split_first_mut()` — returns `(&mut T, &mut [T])` for head/tail patterns

## OCaml Approach

OCaml records allow simultaneous access to multiple fields through references with no restriction:

```ocaml
type game_state = { mutable player_x: float; mutable player_y: float; mutable enemies: (float * float) list }
let update state dx dy =
  state.player_x <- state.player_x +. dx;
  state.player_y <- state.player_y +. dy
```

No borrowing concept exists — all fields are always accessible through the record reference.

## Key Differences

1. **Field-level tracking**: Rust's borrow checker tracks individual struct fields — an improvement from early Rust which tracked whole structs; OCaml has no borrow tracking at all.
2. **Method boundary**: When split-borrow logic is inside a method returning a tuple, Rust accepts it; when callers try to hold borrows and call other methods simultaneously, the checker may reject it.
3. **Slice splits**: Rust requires `split_at_mut` for safe mutable slice splits; OCaml array/Bigarray slices can be subindexed mutably without restriction.
4. **ECS systems**: Rust ECS frameworks (Bevy, Legion) use unsafe code or archetype-based storage to achieve the split-borrow patterns that game logic requires; OCaml ECS frameworks rely on runtime discipline.

## Exercises

1. **Physics update**: Add a method `fn physics_step(&mut self) -> (&mut f32, &mut f32, &[(f32, f32)]) { ... }` and write a loop that moves the player toward the nearest enemy using the split references.
2. **Slice head/tail**: Write a function `fn map_head_tail<T: Clone + std::fmt::Debug>(s: &mut [T], f: impl Fn(&mut T))` using `split_first_mut` that applies `f` to the head while printing the tail.
3. **Struct with two Vecs**: Create a `struct Buffers { input: Vec<u8>, output: Vec<u8> }` and a method returning `(&mut Vec<u8>, &mut Vec<u8>)` — then use both to implement a transform-in-place operation.
