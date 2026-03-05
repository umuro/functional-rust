📖 **[View on hightechmind.io →](https://hightechmind.io/rust/203-lens-laws)**

---

# 203: Lens Laws — What Makes a Lens Well-Behaved

**Difficulty:** ⭐⭐⭐  **Level:** Intermediate

Three simple laws separate a trustworthy Lens from one that will silently corrupt your data.

## The Problem This Solves

A Lens is just two functions. Nothing stops you from writing a `set` function that secretly mutates unrelated fields, or a `get` that returns a transformed value that doesn't round-trip back. Your code will compile. Your tests might even pass — until you compose the Lens with another and the silent transformation accumulates somewhere unexpected.

Without laws, Lens composition becomes a minefield. You can't confidently say "I composed three Lenses; the result is a correct accessor for the deeply nested field." You'd have to inspect every intermediate implementation.

Laws are **contracts that enable composition to be trustworthy**. When every Lens in your system satisfies the three laws, you can compose them freely and the resulting Lens is always correct. Without laws, composition is gambling. This example exists to solve exactly that pain.

## The Intuition

Think of a Lens as a **window into a struct**. A well-behaved window has three properties:

1. **GetSet** — "Looking through the window and then closing it leaves the room unchanged."
   If you `get` a value and immediately `set` it back, the struct is identical to the original.
   ```
   set(get(s), s) == s
   ```

2. **SetGet** — "What you put in is what you see."
   If you `set` a value and then `get` it back, you get exactly what you set.
   ```
   get(set(a, s)) == a
   ```

3. **SetSet** — "The last write wins."
   Setting twice is the same as setting once with the last value — the first set is fully overwritten.
   ```
   set(b, set(a, s)) == set(b, s)
   ```

An unlawful Lens violates at least one of these. The bad example in this file sets `x` but also increments `y` as a side effect — this breaks GetSet immediately because setting `x` to what you just read still changes the struct.

## How It Works in Rust

**The lawful Lens — `x_lens`:**

```rust
fn x_lens() -> Lens<Point, f64> {
    Lens::new(
        |p| p.x,
        |x, p| Point { x, ..p.clone() },   // only x changes
    )
}
```

**The unlawful Lens — `bad_lens`:**

```rust
fn bad_lens() -> Lens<Point, f64> {
    Lens::new(
        |p| p.x,
        |x, p| Point { x, y: p.y + 1.0 },  // set silently mutates y!
    )
}
```

**Verifying the laws as generic functions:**

```rust
// Law 1: GetSet — set what you got changes nothing
fn check_get_set<S: PartialEq + Clone, A: Clone>(lens: &Lens<S, A>, s: &S) -> bool {
    let a = (lens.get)(s);
    (lens.set)(a, s) == *s
}

// Law 2: SetGet — get what you set
fn check_set_get<S: Clone, A: PartialEq + Clone>(lens: &Lens<S, A>, a: A, s: &S) -> bool {
    (lens.get)(&(lens.set)(a.clone(), s)) == a
}

// Law 3: SetSet — last write wins
fn check_set_set<S: PartialEq + Clone, A: Clone>(
    lens: &Lens<S, A>, a: A, b: A, s: &S,
) -> bool {
    (lens.set)(b.clone(), &(lens.set)(a, s)) == (lens.set)(b, s)
}
```

These checkers are generic — they work with any `Lens<S, A>` where `S: PartialEq + Clone` and `A: PartialEq + Clone`. You can use them as property tests or in unit tests with specific values.

**Running the checks:**

```rust
let p = Point { x: 3.0, y: 4.0 };

// x_lens passes all three laws
assert!(check_get_set(&x_lens(), &p));
assert!(check_set_get(&x_lens(), 10.0, &p));
assert!(check_set_set(&x_lens(), 10.0, 20.0, &p));

// bad_lens fails GetSet immediately
assert!(!check_get_set(&bad_lens(), &p));

// Inspect why: set(get(p), p) changed y
let bl = bad_lens();
let p2 = (bl.set)((bl.get)(&p), &p);
assert_eq!(p2.y, 5.0);  // was 4.0 — bad_lens mutated it!
```

## What This Unlocks

- **Safe composition**: if every Lens in your system passes these three checks, any composition of them is also correct — no need to audit the composed result separately.
- **Property testing foundation**: these three checkers drop directly into `proptest` or `quickcheck` for exhaustive random verification.
- **Catching bugs early**: unlawful Lenses usually introduce subtle bugs that only surface when composed — catching them at the law-check level is far easier than debugging composed behaviour.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Equality check | Structural equality by default | Requires `#[derive(PartialEq)]` |
| Law checker signature | `'a lens -> 's -> bool` | `fn check_get_set<S: PartialEq+Clone, A: Clone>` |
| Float comparison | Structural `=` works | `f64: PartialEq` — works, but watch NaN |
| Batch verification | `List.for_all check values` | Iterator `.all(…)` or a loop |
| Bad Lens example | Mutate `y` in `set` on `x` | Same pattern — `y: p.y + 1.0` in the `set` closure |
