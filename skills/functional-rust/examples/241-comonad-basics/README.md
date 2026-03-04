# 241: Comonad Basics

**Difficulty:** 4  **Level:** Expert

The dual of a Monad — instead of injecting values into a context, you extract values from a context and extend computations across it.

## The Problem This Solves

Some computations are inherently context-dependent. A moving average needs the neighbours of each element. A spell-checker needs surrounding words. Game of Life needs adjacent cells. Sliding window statistics need a fixed-size view of recent data.

The imperative approach is messy: keep a mutable window, manually manage indices, copy slices around. The functional approach using `map` doesn't help either — `map` replaces each element in isolation, with no access to neighbours.

You might reach for `windows()` in iterators, but that only works for fixed-size windows. What about variable contexts? What about 2D grids? What about an algorithm that needs to apply the same "look at neighbourhood" operation repeatedly?

Comonads are the abstraction for exactly this pattern: a data structure that always has a "focus" (the current element) plus surrounding context, and an operation `extend` that applies a context-aware function to every possible focus position simultaneously. Sliding windows, cellular automata, image convolutions, Game of Life — all comonadic. This exists to solve exactly that pain.

## The Intuition

You already know **Monad**: wrap a value in a context (like `Option` or `Result`), then chain computations that produce more wrapped values. The key operation is `bind`/`flat_map`: go from `M<A>` to `M<B>` by applying `A -> M<B>`.

**Comonad** is the mirror image — flip all the arrows:
- Monad: `return :: A -> M<A>` (put value into context)  
  Comonad: `extract :: W<A> -> A` (pull value out of context)
- Monad: `bind :: M<A> -> (A -> M<B>) -> M<B>`  
  Comonad: `extend :: W<A> -> (W<A> -> B) -> W<B>`

The difference with `extend`: instead of `A -> M<B>`, you pass `W<A> -> B`. Your function receives the **whole context**, not just the value. And `extend` applies it to every possible subcontext of the structure.

Think of a spreadsheet: each cell has a value. The "context" is the cell's position and its neighbours. `extract` reads the current cell. `extend` applies a formula (like SUM of neighbours) to every cell simultaneously, producing a new spreadsheet where each cell's value is the result of the formula applied at that position.

**The Zipper** is the canonical comonad: a list with a "cursor" position. `extract` reads the cursor element. `move_left`/`move_right` shift the cursor. `extend` applies a function at every cursor position, building a new zipper where each position's value came from looking at its neighbourhood.

```
Original zipper:  [1, 2, |3|, 4, 5]  (focus = 3)
extend(sum_neighbors):
  at position 1: 0 + 1 + 2 = 3
  at position 2: 1 + 2 + 3 = 6
  at position 3: 2 + 3 + 4 = 9   <- this is the new focus value
  at position 4: 3 + 4 + 5 = 12
  at position 5: 4 + 5 + 0 = 9
Result zipper:    [3, 6, |9|, 12, 9]
```

## How It Works in Rust

```rust
// Identity comonad — the simplest case
#[derive(Clone)]
struct Identity<A>(A);

impl<A: Clone> Identity<A> {
    // extract: get the value
    fn extract(&self) -> A { self.0.clone() }
    
    // extend: apply f to this context (trivial here — only one "position")
    fn extend<B>(&self, f: impl Fn(&Identity<A>) -> B) -> Identity<B> {
        Identity(f(self))  // f receives the whole Identity, returns B
    }
    
    // duplicate: Identity<A> -> Identity<Identity<A>>
    // "unfocus" — each position now holds the store focused there
    fn duplicate(&self) -> Identity<Identity<A>> {
        Identity(Identity(self.0.clone()))
    }
}

// Zipper comonad — list with a focused position
#[derive(Debug, Clone)]
pub struct Zipper<A> {
    pub left: Vec<A>,   // elements to the left of focus (nearest first)
    pub focus: A,       // the currently focused element
    pub right: Vec<A>,  // elements to the right
}

impl<A: Clone> Zipper<A> {
    // Comonad law 1: extract gets the focused element
    pub fn extract(&self) -> A { self.focus.clone() }

    // extend: apply f at EVERY position, build a new Zipper
    pub fn extend<B: Clone>(&self, f: impl Fn(&Zipper<A>) -> B) -> Zipper<B> {
        // Generate all possible focus positions
        let all_positions = self.all_positions();
        let focus_idx = self.left.len();  // our position in the list
        
        // Apply f at each position — f sees the whole zipper context
        let values: Vec<B> = all_positions.iter().map(|z| f(z)).collect();
        
        // Rebuild zipper with new values, same structure
        Zipper {
            left: values[..focus_idx].iter().cloned().rev().collect(),
            focus: values[focus_idx].clone(),
            right: values[focus_idx + 1..].to_vec(),
        }
    }
}

// Usage: moving average via extend
let z = Zipper::from_slice(&[1, 2, 3, 4, 5]).unwrap();

let moving_avg = |z: &Zipper<i32>| {
    let left_val = z.move_left().map(|l| l.focus).unwrap_or(z.focus);
    let right_val = z.right.first().copied().unwrap_or(z.focus);
    (left_val + z.focus + right_val) / 3
};

let avg_z = z.extend(moving_avg);
// avg_z.to_vec() = [1, 2, 3, 4, 4]  (3-element moving average)
```

**Comonad law to verify:** `extract(extend(f)(w)) == f(w)` — applying `extend` and immediately `extract`ing gives the same as just calling `f` on the original structure.

## What This Unlocks

- **Cellular automata / Game of Life:** Each cell's next state depends on neighbours. Model as a `Zipper` (1D) or 2D grid comonad; `extend rule` applies the rule everywhere simultaneously.
- **Image processing convolutions:** A 2D comonad where each pixel's value is computed from its neighbourhood (blur, sharpen, edge-detect) — `extend kernel` applies the kernel to every pixel.
- **Sliding window statistics:** Moving average, rolling max, time-series features — all reducible to `extend f` over a zipper or stream comonad.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `extract` | `val extract : 'a t -> 'a` | `fn extract(&self) -> A where A: Clone` |
| `extend` | `val extend : ('a t -> 'b) -> 'a t -> 'b t` | `fn extend<B>(&self, f: impl Fn(&Self) -> B) -> ...` |
| Zipper movement | Pattern match on variant | `move_left`/`move_right` return `Option<Zipper<A>>` |
| `all_positions` | Recursive list traversal | Collect via repeated `move_right` calls |
| Ownership in `extend` | GC handles sharing | `Clone` required; all positions cloned before applying `f` |
| Trait | No standard comonad typeclass in stdlib | No standard comonad trait in Rust either; implement per-type |
