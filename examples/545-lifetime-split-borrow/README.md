📖 **[View on hightechmind.io →](https://hightechmind.io/rust/545-lifetime-split-borrow)**

---

# 545: Split Borrows from Structs

**Difficulty:** 4  **Level:** Advanced

The borrow checker tracks field granularity. You can mutably borrow two *different* fields of a struct at the same time — the compiler knows they don't overlap.

## The Problem This Solves

A common frustration: you have a struct with two fields and need to pass mutable references to both simultaneously:

```rust
struct State {
    data: Vec<i32>,
    cache: Vec<i32>,
}

let mut s = State { data: vec![1,2,3], cache: vec![] };

// Intuitive but fails:
process(&mut s.data, &mut s.cache);
// ERROR: cannot borrow `s` as mutable more than once
// The compiler sees TWO borrows of `s` — same source
```

But this isn't actually unsafe — `data` and `cache` are different memory locations. The borrow checker handles this, but only when you borrow fields *directly*, or use methods that split the struct for you.

## The Intuition

The borrow checker reasons about *paths*, not just variables. `s.data` and `s.cache` are different paths into `s`. Borrowing both mutably is safe — the same reason you can write to two different elements of an array (as long as they don't alias).

The key is that the *struct itself* can't be borrowed mutably twice — but its *fields* can be, because each field has a unique address. You need to do the split at the borrow level, not after the fact.

Methods can formalize split borrows by returning `(&mut T, &mut U)` from a single `&mut self` call. The compiler can verify that the two returned mutable references point to different parts of `self`.

## How It Works in Rust

**Direct field access — compiler sees separate paths:**

```rust
struct GameState {
    player_x: f32,
    enemies: Vec<(f32, f32)>,
    score: u32,
}

fn update(gs: &mut GameState) {
    let px = gs.player_x; // read player_x (Copy — no borrow)
    
    // Borrow enemies (different field from score)
    let nearby = gs.enemies.iter()
        .filter(|&&(ex, _)| (ex - px).abs() < 10.0)
        .count();
    
    // Mutate score (different field from enemies)
    gs.score += nearby as u32 * 10; // OK — score ≠ enemies
}
```

**Method returning split mutable borrows:**

```rust
impl GameState {
    // Returns (&mut f32, &mut f32, &mut u32) — three different fields
    fn position_and_score_mut(&mut self) -> (&mut f32, &mut f32, &mut u32) {
        (&mut self.player_x, &mut self.player_y, &mut self.score)
        // Compiler verifies: player_x, player_y, score are disjoint fields
    }
}

let (px, py, score) = gs.position_and_score_mut();
*px = 5.0;
*py = 3.0;
*score += 100;
// All three modified — no conflict because they're separate fields
```

**`split_at_mut` — the canonical slice split:**

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];

// v.split_at_mut(3) returns (&mut [i32], &mut [i32]) — non-overlapping halves
let (left, right) = v.split_at_mut(3);
left[0] += 100;  // modifying left half
right[0] += 200; // modifying right half — safe!
// This would be an ERROR with indexed borrows: v[0] and v[3] in same borrow

println!("{:?}", v); // [101, 2, 3, 204, 5, 6]
```

**When split borrows don't work — same field:**

```rust
// Can't split if both borrows are of the same field:
let (a, b) = (&mut s.data, &mut s.data); // ERROR: two mutable borrows of same field
// Fix: use s.data.split_at_mut() instead
```

## What This Unlocks

- **Parallel processing within a single struct** — pass mutable references to two halves of a game state to two different subsystems without copying or unsafe code.
- **Cache-friendly struct layouts** — keep hot and cold data in the same struct, use split borrows to access hot data while cold data is simultaneously processed.
- **Avoiding unnecessary `Arc<Mutex<T>>`** — when you control the entire struct and just need to touch two fields, split borrows give you safe concurrent-*seeming* access on a single thread without synchronization overhead.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable access to record fields | Records are immutable by default; `ref` for mutation; GC manages | `&mut` is exclusive — but split borrows let you hold `&mut` to two *different* fields |
| Simultaneous field mutation | Update syntax creates new record; no aliasing concern | Direct field split: compiler verifies disjointness at field level |
| Slice splitting | `Array.sub` allocates; manual indexing | `split_at_mut` returns `(&mut [T], &mut [T])` — zero cost, borrow-checked |
| Interior mutability | `ref` cells or mutable fields | Split borrows: when fields are known-disjoint, `&mut` to both is safe |
| Safety | Mutation tracked by discipline and GC | Field-level borrow tracking: unsafe would be needed only for dynamic disjointness (e.g., two arbitrary indices) |
