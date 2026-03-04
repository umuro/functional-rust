# 242: Store Comonad

**Difficulty:** 4  **Level:** Expert

Model any "array-like" structure as a getter function plus a focus position — then apply context-aware computations across every position at once.

## The Problem This Solves

Writing a 1D cellular automaton (like Conway's Game of Life in 1D, or Rule 30) in Rust is straightforward but clunky. You need an array, index arithmetic, wrap-around logic, and a loop that simultaneously reads the old generation while writing the new one:

```rust
let mut next = vec![false; n];
for i in 0..n {
    let left  = cells[(i + n - 1) % n];
    let right = cells[(i + 1) % n];
    next[i] = rule(left, cells[i], right);
}
cells = next;
```

This works, but every algorithm that touches neighbours duplicates this pattern. The wrap-around logic is copied everywhere. And the mutation — "read old, write new" — is error-prone.

What if you could write the rule for a *single* cell and have the runtime apply it to all cells simultaneously? What if "read the neighbour" was just a function call, and the wrap-around was handled once in one place?

The Store comonad does exactly this. You define the entire cellular automaton as a single function `i -> bool` (given an index, return whether that cell is alive). The `extend` operation applies a neighbourhood rule to every position at once, producing a new function that describes the next generation — no mutation, no index arithmetic in the rule itself. This exists to solve exactly that pain.

## The Intuition

A **Store** is two things:
1. A **getter function** `S -> A`: given any position/key of type `S`, return the value `A` there
2. A **current focus** of type `S`: the position we're currently "at"

Think of it like a spreadsheet cursor. The spreadsheet is the getter function (given any `(row, col)`, return the cell value). The cursor is your current position. You can:
- `extract`: read the value at the cursor
- `peek(pos)`: read any other position without moving the cursor
- `seek(pos)`: move the cursor to a new position
- `extend(f)`: create a new spreadsheet where each cell's value is `f` applied with the cursor at that cell

The power of `extend`: your function `f` receives a Store focused at each position in turn, can call `peek` to read neighbours, and returns one value. `extend` builds a new getter function that does this for every possible position — lazily, without actually iterating until you ask.

```
Original store: getter = |i| i * i,  focus = 4
  extract()   → 16          (4² = 16)
  peek(3)     → 9           (3² = 9)
  seek(7)     → store focused at 7
  
extend(|s| s.extract() + 1):
  new getter = |i| (i*i) + 1
  new store focus still at 4
  new extract() → 17
```

## How It Works in Rust

```rust
// Store<S, A>: a getter function (S -> A) plus a focus position S
#[derive(Clone)]
pub struct Store<S, A> {
    getter: Rc<dyn Fn(S) -> A>,  // Rc for cheap cloning
    position: S,                  // current focus
}

impl<S: Clone + 'static, A: Clone + 'static> Store<S, A> {
    pub fn new(getter: impl Fn(S) -> A + 'static, position: S) -> Self {
        Store { getter: Rc::new(getter), position }
    }

    // extract: read value at current focus
    pub fn extract(&self) -> A { (self.getter)(self.position.clone()) }

    // peek: read value at any position (without moving)
    pub fn peek(&self, s: S) -> A { (self.getter)(s) }

    // seek: create new store focused at different position
    pub fn seek(&self, s: S) -> Store<S, A> {
        Store { getter: self.getter.clone(), position: s }
    }

    // extend: build new store by applying f at every possible position
    // f receives a Store focused at that position — can call extract, peek, etc.
    pub fn extend<B: Clone + 'static>(
        &self,
        f: impl Fn(&Store<S, A>) -> B + 'static,
    ) -> Store<S, B> {
        let original = self.clone();
        let f = Rc::new(f);
        Store {
            getter: Rc::new(move |s: S| {
                let focused_here = original.seek(s);  // store focused at position s
                f(&focused_here)                       // apply f with that focus
            }),
            position: self.position.clone(),
        }
    }
}

// Build a cellular automaton store from a fixed array (with wrap-around)
fn make_cell_store(cells: &[bool]) -> Store<i32, bool> {
    let cells = cells.to_vec();
    let len = cells.len() as i32;
    Store::new(
        move |i: i32| {
            let idx = ((i % len) + len) % len;  // wrap-around indexing — defined ONCE
            cells[idx as usize]
        },
        0,  // start focused at cell 0
    )
}

// Rule 30: define the rule for ONE cell — extend handles the rest
fn rule30_step(store: &Store<i32, bool>) -> bool {
    let i = store.pos();
    let left   = store.peek(i - 1);  // read neighbour — wrap-around handled by getter
    let center = store.extract();     // read current focus
    let right  = store.peek(i + 1);  // read other neighbour
    // Rule 30: XOR-like rule
    matches!((left, center, right),
        (false, false, true) | (false, true, false) | (false, true, true) | (true, false, false)
    )
}

// Run the automaton: one extend call per generation
let mut s = make_cell_store(&initial_cells);
for _ in 0..5 {
    s = s.extend(rule30_step);  // new generation — no mutation, no index arithmetic
}
```

The elegance: `rule30_step` doesn't know or care about wrap-around. It just calls `store.peek(i-1)` and `store.peek(i+1)`. The wrap-around is in the getter, defined once. `extend` applies the rule at every position.

## What This Unlocks

- **Cellular automata / Game of Life:** Write the rule for one cell (`extend rule`), run multiple generations with repeated `extend`. No mutable arrays needed.
- **Image filtering / convolution:** A 2D Store where `peek(x, y)` reads any pixel. `extend blur_kernel` applies the blur to every pixel simultaneously.
- **Functional lenses:** The Store comonad is the foundation of lenses — `get` is `extract`, `set` rebuilds the getter. This connection explains why lens libraries compose so cleanly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Store type | `type ('s, 'a) store = Store of ('s -> 'a) * 's` | `struct Store<S, A> { getter: Rc<dyn Fn(S)->A>, position: S }` |
| Sharing getter | GC references | `Rc` for cheap clone in `seek` and `extend` |
| `extend` | Builds new `store` value | Builds new `Store` with new `Rc<dyn Fn>` |
| Index wrap-around | Handled in getter, closed over | Same — closure captures array and length |
| Comonad law | Equational proofs | Tested: `extract(extend(f)(s)) == f(s)` |
| Cellular automaton | `let next_gen = Store.extend rule30_step current` | `s = s.extend(rule30_step)` |
