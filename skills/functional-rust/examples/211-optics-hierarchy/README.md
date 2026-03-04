# 211: Optics Hierarchy

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Every Lens is a Traversal, every Iso is a Lens — understand how the optic types relate, and write generic functions that work across the full hierarchy.

## The Problem This Solves

You're working with a library that has several optic types: `Lens`, `Prism`, `Traversal`, `Iso`. You want to pass a `Lens` to a function that expects a `Traversal`. You want to know: can you? Should you? What do you lose?

Without understanding the hierarchy, you end up with duplicated code — one version of "collect all focused values" for `Lens`, another for `Traversal`, a third for `Prism`. Or you end up writing wrapper types manually every time you need to up-cast.

The optics hierarchy exists because each optic type is a *specialisation* of the one above it. Understanding the hierarchy lets you write generic functions once (`fn view`, `fn modify`, `fn collect`) that work at any level — and lets the type system enforce which operations are valid for each optic type. This exists to solve exactly that pain.

## The Intuition

Here is the full hierarchy, from most specific to most general:

```
         Iso
        /   \
      Lens  Prism
        \   /
       Traversal
```

- An **Iso** (isomorphism) is a lossless two-way conversion. `Celsius ↔ f64`. You can always get and always set, with no information loss. Every Iso is a Lens *and* a Prism.
- A **Lens** focuses on **exactly one** value inside a product type (struct). You always get something. `Point.x`, `User.name`.
- A **Prism** focuses on **zero or one** value inside a sum type (enum). It might fail to extract. `Color::Red`, `Ok(v)` inside `Result`.
- A **Traversal** focuses on **zero or more** values. It's the most general. Every Lens and Prism is a degenerate Traversal.
- An **Affine Traversal** sits between Prism and Traversal — at most one focus, like a Lens that can fail.

**The subtype relationships:**

| Optic | Can be used as | Because |
|-------|---------------|---------|
| Iso | Lens, Prism, Traversal | It's both; works for products and sums |
| Lens | Traversal | "exactly 1 focus" is a special case of "zero or more" |
| Prism | Traversal | "zero or one focus" is a special case of "zero or more" |

**Analogy:** Think of SQL. `WHERE id = 5` (Lens — exactly one row) is a special case of `WHERE age > 30` (Traversal — many rows). You can always use a single-row result where many rows are expected. The hierarchy is the same idea: every more-specific optic "is a" more-general optic.

In Rust, we encode the hierarchy with traits:

```
Getter<S, A>  ←  can read one value
    +
Setter<S, A>  ←  can write one value
    ↓
LensLike<S, A>  ←  Lens: both getter and setter

PrismLike<S, A>  ←  preview + review

ToList<S, A>  ←  can list all focused values (Traversal level)
```

## How It Works in Rust

```rust
// Step 1: Trait hierarchy — each trait adds one capability
trait Getter<S, A> {
    fn get(&self, s: &S) -> A;
}

trait Setter<S, A> {
    fn set(&self, a: A, s: &S) -> S;
    // over is derived: get then set
    fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S where Self: Getter<S, A> {
        self.set(f(self.get(s)), s)
    }
}

// Lens: has both Getter and Setter (exactly one focus)
trait LensLike<S, A>: Getter<S, A> + Setter<S, A> {}

// Prism: might fail to extract, but can always construct
trait PrismLike<S, A> {
    fn preview(&self, s: &S) -> Option<A>;  // extract: might be None
    fn review(&self, a: A) -> S;            // construct: always works
}

// Traversal level: list all focused values
trait ToList<S, A> {
    fn to_list(&self, s: &S) -> Vec<A>;
}

// Step 2: Concrete types at each level

// Iso: Celsius ↔ f64 — bidirectional, no information loss
struct CelsiusIso;
impl Getter<Celsius, f64> for CelsiusIso { fn get(&self, s: &Celsius) -> f64 { s.0 } }
impl Setter<Celsius, f64> for CelsiusIso { fn set(&self, a: f64, _: &Celsius) -> Celsius { Celsius(a) } }
impl LensLike<Celsius, f64> for CelsiusIso {}  // Iso → Lens: automatic

// Lens: Point.x — always exactly one f64 in a Point
struct XLens;
impl Getter<Point, f64> for XLens { fn get(&self, s: &Point) -> f64 { s.x } }
impl Setter<Point, f64> for XLens { fn set(&self, a: f64, s: &Point) -> Point { Point { x: a, ..s.clone() } } }
impl LensLike<Point, f64> for XLens {}
// Lens → Traversal: wrap the single focus in a Vec
impl ToList<Point, f64> for XLens {
    fn to_list(&self, s: &Point) -> Vec<f64> { vec![self.get(s)] }  // exactly one element
}

// Prism: Color::Red — matches or doesn't
struct RedPrism;
impl PrismLike<Color, ()> for RedPrism {
    fn preview(&self, s: &Color) -> Option<()> {
        match s { Color::Red => Some(()), _ => None }  // fails for non-Red
    }
    fn review(&self, _: ()) -> Color { Color::Red }    // always constructs Red
}
// Prism → Traversal: zero elements if no match, one if match
impl ToList<Color, ()> for RedPrism {
    fn to_list(&self, s: &Color) -> Vec<()> {
        self.preview(s).into_iter().collect()  // [] or [()]
    }
}

// Step 3: Generic functions that work at each level
fn view<S, A>(getter: &impl Getter<S, A>, s: &S) -> A { getter.get(s) }
fn modify<S, A>(lens: &(impl Getter<S, A> + Setter<S, A>), f: impl FnOnce(A) -> A, s: &S) -> S { ... }
fn collect<S, A>(t: &impl ToList<S, A>, s: &S) -> Vec<A> { t.to_list(s) }

// Same generic function works for Iso, Lens, and anything implementing ToList
collect(&XLens, &Point { x: 3.0, y: 4.0 });  // [3.0]  — Lens used as Traversal
collect(&RedPrism, &Color::Red);              // [()]   — Prism used as Traversal
collect(&RedPrism, &Color::Blue);             // []     — Prism: no match
```

## What This Unlocks

- **Write code once, use at any level** — generic functions bounded by `impl Getter` or `impl ToList` automatically accept Isos, Lenses, Prisms — the type system enforces correctness without boilerplate conversions.
- **Optic libraries** — when using or building an optics library (like `lens` crates), understanding the hierarchy tells you which optic to reach for: mandatory field → Lens, enum variant → Prism, collection → Traversal.
- **Profunctor optics** — the profunctor encoding (example 621) reflects this same hierarchy via type class constraints: `Strong` profunctor = Lens, `Choice` profunctor = Prism, `Traversing` profunctor = Traversal.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Hierarchy encoding | Sum type `optic` + explicit conversion functions | Trait hierarchy — automatic via `impl Trait` bounds |
| Subtyping | `lens_to_traversal : lens -> traversal` explicit coercions | Automatic: `impl LensLike` also satisfies `impl Getter + Setter` |
| Generic functions | Functions take specific optic type + pass as arguments | `impl Trait` bounds — monomorphised at compile time |
| Zero-cost abstractions | Closures in records — some overhead | Trait dispatch: zero-cost when monomorphised, small overhead with `dyn` |
| Iso encoding | Pair of functions `(get, set)` both ways | Struct implementing both `Getter + Setter` (no separate "review" needed for Lens direction) |
