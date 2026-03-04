# 244: Comonad Laws

**Difficulty:** 5  **Level:** Master

Verify the three comonad laws on the Stream comonad — and understand what they mean.

## The Problem This Solves

Laws aren't just mathematical vanity. When you implement a comonad (or use a library that claims to provide one), the laws are contracts that let you *refactor safely*. If `extend extract = id`, you can remove a no-op extend without changing behavior. If the associativity law holds, you can split a complex extend into two simpler ones — or merge them — without fear.

The classic abuse case: you implement a moving average over a data stream. You chain several `extend` calls. Are you applying them in the right order? Can you fuse two passes into one? The comonad laws tell you exactly what substitutions are safe.

The **Stream comonad** is the ideal testbed because it's infinite, it has a clear notion of "current position," and every comonad law has a direct intuitive meaning in terms of time-series data.

## The Intuition

The **Stream comonad** is an infinite lazy list with a "current position":
- `Stream<A>` = the current value (head) plus a lazy tail generating the rest
- `extract` = read the current head value
- `extend f` = apply `f` to *every suffix* of the stream, not just the current element

The key insight: `map` applies `f` to each element. `extend` applies `f` to each element *and all its context* (everything after it). For a moving average, `f` needs to see the next 3 elements — that's context. `extend` provides it.

**The three laws (in plain English):**

1. **Left identity** — `extract(extend f s) = f(s)`: If you extend with `f` then immediately extract, you get the same result as just calling `f` on the original stream. *The current position is preserved.*

2. **Right identity** — `extend extract s = s`: If you extend with the trivial function "just read your own head," you get back the same stream unchanged. *The identity computation does nothing.*

3. **Associativity** — `extend f (extend g s) = extend (f . extend g) s`: The order of chaining extends matches what you'd get by composing the computations before extending. *You can fuse or split passes.*

`duplicate` is the "canonical" comonad operation from which `extend` can be derived: `duplicate s` produces a stream *of streams*, where position `i` holds the stream starting at `i`. `extend f = map f . duplicate`.

## How It Works in Rust

The Stream is defined as a recursive struct with a lazy tail:
```rust
pub struct Stream<A: Clone> {
    pub head: A,
    tail: Rc<dyn Fn() -> Stream<A>>,  // lazy: only computed when accessed
}
```

`extend` applies `f` to every suffix lazily — only computes when the stream is consumed:
```rust
pub fn extend<B>(&self, f: Rc<dyn Fn(&Stream<A>) -> B>) -> Stream<B> {
    let head_val = f(self);           // apply f to current suffix
    let tail_stream = self.tail();
    let f_clone = f.clone();
    // Recursively extend the tail — lazy, not immediately evaluated
    Stream::new(head_val, move || tail_stream.extend(f_clone.clone()))
}
```

Moving average (needs 3-element context — only possible with `extend`, not `map`):
```rust
let avg = s.extend(Rc::new(|st: &Stream<i64>| {
    let t = st.tail();
    let tt = t.tail();
    (st.head + t.head + tt.head) / 3  // st has access to its own context
}));
```

Law verification (checking first N elements for the infinite stream):
```rust
// Law 1: extract(extend f s) == f(s)
fn check_law1(s: &Stream<i64>, f: Rc<..>) -> bool {
    s.extend(f.clone()).extract() == f(s)
}

// Law 2: extend extract == id (first N elements equal)
fn check_law2(s: &Stream<i64>, n: usize) -> bool {
    s.extend(Rc::new(|st: &Stream<i64>| st.extract())).take(n) == s.take(n)
}
```

## What This Unlocks

- **Signal processing and time-series**: any "windowed" computation (moving average, local extrema detection, pattern matching) is an `extend` on a Stream comonad.
- **Cellular automata**: Conway's Game of Life is `extend` on a 2D grid comonad — each cell's new state depends on its neighborhood.
- **Safe refactoring of pipelines**: if your data pipeline obeys the comonad laws, you can restructure the chain of transforms without changing output.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Infinite stream | `type 'a stream = { head: 'a; tail: unit -> 'a stream }` | `struct Stream<A> { head: A, tail: Rc<dyn Fn() -> Stream<A>> }` |
| Lazy tail | Closure `unit ->` | `Rc<dyn Fn()>` — shared ownership needed for recursive cloning |
| extend | Higher-kinded via module | Method with `Rc<dyn Fn>` to allow cloning |
| Sharing tails | GC handles aliasing | `Rc` for reference counting; `Arc` for threads |
| Comonad typeclass | First-class typeclass | Implement methods directly; no HKT in stable Rust |
