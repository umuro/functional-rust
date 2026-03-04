# 218: Anamorphism — Build a Structure From a Seed

**Difficulty:** ⭐⭐⭐  **Category:** Recursion Schemes

The opposite of a fold: start with a single value and grow an entire recursive structure from it.

## The Problem This Solves

Some algorithms don't consume a structure — they *produce* one. You have a seed value and rules for expanding it step by step:

- Start at `1`, keep adding 1 until you hit `100` → range list
- Start at `n`, apply 3n+1 or n/2 until you hit 1 → Collatz sequence
- Start with a depth number, keep splitting → balanced binary tree
- Start with a grammar rule, keep expanding → parse tree

Each of these follows the same shape:
1. Look at the current seed
2. Decide: stop here, or produce a value and a new seed for the next step
3. Repeat with the new seed

Here's the problem: when you write this by hand, you're once again mixing "the stopping/expansion logic" with "how to build the structure". You end up with slightly different recursive patterns for each case, none of which compose naturally.

The anamorphism pattern (also called **unfold**) separates these. You write a **coalgebra** — a function that takes a seed and returns "stop, or here's the next value plus a new seed". The `ana` function handles everything else: recursing, building the structure, terminating.

This is exactly what `Iterator::from_fn` does for flat sequences. Anamorphisms generalize it to trees and any other recursive structure.

## The Intuition

A coalgebra (the anamorphism equivalent of an algebra) is dead simple. It answers one question for each seed value: *what does this step look like?*

For a range `[1..5]`:
- Seed 1: produce 1, next seed is 2
- Seed 2: produce 2, next seed is 3
- Seed 5: produce 5, next seed is 6
- Seed 6: stop (6 > 5)

```rust
let range_coalg = |s: (i64, i64)| {
    if s.0 > s.1 {
        ListF::NilF                          // stop
    } else {
        ListF::ConsF(s.0, (s.0 + 1, s.1))   // produce s.0, advance
    }
};
```

That's it. The `ana` function takes this coalgebra and a seed, and builds the whole list:

```rust
let range_1_to_5 = ana(&range_coalg, (1, 5));
// Result: 1 → 2 → 3 → 4 → 5 → Nil
```

Compare to `Iterator::from_fn`:

```rust
// Iterator version (flat sequence)
let range = std::iter::from_fn({
    let mut i = 1;
    move || if i > 5 { None } else { let v = i; i += 1; Some(v) }
});
```

Same idea — anamorphisms just work for trees and any other shaped structure, not just linear sequences.

## How It Works in Rust

**`ana` — the universal unfold:**

```rust
#[derive(Debug, Clone)]
enum ListF<A> { NilF, ConsF(i64, A) }

impl<A> ListF<A> {
    // Apply f to every child position
    fn map<B>(self, f: impl Fn(A) -> B) -> ListF<B> {
        match self {
            ListF::NilF        => ListF::NilF,
            ListF::ConsF(x, a) => ListF::ConsF(x, f(a)),
        }
    }
}

struct FixList(Box<ListF<FixList>>);

fn ana<S>(coalg: &dyn Fn(S) -> ListF<S>, seed: S) -> FixList {
    // 1. Run the coalgebra: what does this seed produce?
    // 2. .map() recurses into each child seed, building the rest
    // 3. Wrap the result in FixList
    FixList(Box::new(coalg(seed).map(|s| ana(coalg, s))))
}
```

**Coalgebra examples:**

```rust
// Range [lo..=hi]
fn range(lo: i64, hi: i64) -> FixList {
    ana(&|s: (i64, i64)| {
        if s.0 > s.1 { ListF::NilF }                          // stop
        else { ListF::ConsF(s.0, (s.0 + 1, s.1)) }            // next
    }, (lo, hi))
}

// Countdown 5 → 4 → 3 → 2 → 1 → stop
fn countdown(n: i64) -> FixList {
    ana(&|s| {
        if s <= 0 { ListF::NilF }
        else { ListF::ConsF(s, s - 1) }
    }, n)
}

// Collatz sequence: n → n/2 (even) or 3n+1 (odd), stop at 1
fn collatz(n: i64) -> FixList {
    ana(&|s| {
        if s <= 0          { ListF::NilF }
        else if s == 1     { ListF::ConsF(1, 0) }              // produce 1, then stop
        else if s % 2 == 0 { ListF::ConsF(s, s / 2) }
        else               { ListF::ConsF(s, 3 * s + 1) }
    }, n)
}
```

**Tree anamorphism — works exactly the same way:**

```rust
enum TreeF<A> { LeafF(i64), BranchF(A, A) }

struct FixTree(Box<TreeF<FixTree>>);

fn ana_tree<S>(coalg: &dyn Fn(S) -> TreeF<S>, seed: S) -> FixTree {
    FixTree(Box::new(coalg(seed).map(|s| ana_tree(coalg, s))))
}

// Build a balanced binary tree of given depth
fn balanced_tree(depth: u32) -> FixTree {
    ana_tree(&|s: (u32, i64)| {
        if s.0 == 0 {
            TreeF::LeafF(s.1)  // stop: leaf with this value
        } else {
            // split: left child gets same depth-1, right child gets different value
            TreeF::BranchF(
                (s.0 - 1, s.1),
                (s.0 - 1, s.1 + (1i64 << (s.0 - 1))),
            )
        }
    }, (depth, 1))
}
```

**Reading the results back:**

```rust
assert_eq!(to_vec(&range(1, 5)),    vec![1, 2, 3, 4, 5]);
assert_eq!(to_vec(&countdown(5)),   vec![5, 4, 3, 2, 1]);
assert_eq!(to_vec(&collatz(6)),     vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
assert_eq!(tree_to_vec(&balanced_tree(2)), vec![1, 2, 3, 4]);
```

## What This Unlocks

- **Generate any recursive structure from a simple rule.** Ranges, sequences, trees, tries, graphs — all expressible as coalgebras. Your generation logic stays separate from the structure type.
- **Build the input for a `cata`.** `ana` builds a structure; `cata` folds it. Stack them together (with `hylo`) to express algorithms as "build then consume" pipelines — like compilers do.
- **Controlled termination.** The coalgebra decides when to stop by returning a leaf/nil variant. You can prove termination by reasoning about just the coalgebra, not the recursive machinery.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `ana` | `let rec ana coalg seed = FixL (map_lf (ana coalg) (coalg seed))` | Same structure with `&dyn Fn` |
| Coalgebra type | `'seed -> 'seed list_f` | `S -> ListF<S>` (identical idea) |
| Termination | Return `NilF` / `LeafF` | Same — return the base variant |
| Seed type | Any polymorphic type | Any `S` — tuples work great for multi-value seeds |
| Building output | Direct pattern construction | Same, but `Box::new` for heap nodes |
