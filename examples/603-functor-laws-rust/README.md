# 603: Functor Laws in Practice

**Difficulty:** 5  **Level:** Master

Implement the Functor trait for `Option`, `Result`, and `Vec`, then verify both Functor laws hold — the same laws that make `map` trustworthy across Rust's entire standard library.

## The Problem This Solves

You've been using `.map()` on `Option`, `Result`, and `Vec` for years. But have you ever wondered: *why can you chain them safely? Why does replacing two `.map()` calls with one always work? Why does the compiler never warn you that `.map(|x| x)` is a no-op?*

The answer is that these types all follow two precise rules — the Functor laws. Without these rules, "map" would just be a method name with unpredictable behavior. With them, `map` has a contract you can rely on.

```rust
// You probably write this all the time:
let result = some_option
    .map(parse_string)
    .map(validate_value)
    .map(format_output);

// You trust this is equivalent to:
let result = some_option.map(|s| format_output(validate_value(parse_string(s))));
```

That trust is the Functor composition law. This example cracks open `Option`, `Result`, and `Vec` at the trait level — you implement the `Functor` trait yourself, verify both laws, and see exactly why the standard library's `map` is reliable. This concept exists to solve exactly that pain: understanding *why* map-heavy Rust code is safe to refactor.

## The Intuition

Think of a Functor like a labeled shipping container. The label says what's inside (the type). You can change what's inside without relabeling — that's `map`. Two rules must hold:

**Law 1 — Identity:** Relabeling with "keep everything the same" should produce an identical container. `container.map(|x| x)` must equal `container`. If a map changes *anything* when you give it an identity function, the container has hidden state. That's a bug.

**Law 2 — Composition:** Two relabeling trips should produce the same result as one combined trip. `.map(f).map(g)` must equal `.map(|x| g(f(x)))`. If they differ, your container is accumulating state between maps — again, hidden behavior.

These laws together mean: **map can only transform values, never the container's structure, count, or metadata.** No side effects. No surprises.

In Rust's standard library, these laws are upheld by design for `Option`, `Vec`, and `Result`. This example shows you how to verify that yourself — and how to apply the same discipline to your own types.

## How It Works in Rust

**Step 1 — Define the Functor trait using GATs:**

```rust
trait Functor {
    type Inner;          // the A in Container<A>
    type Mapped<B>;      // the Container<B> you get after mapping
    fn fmap<B>(self, f: impl FnMut(Self::Inner) -> B) -> Self::Mapped<B>;
}
```

GATs (Generic Associated Types) are what make `type Mapped<B>` possible — they let the trait express "a version of myself holding type B instead of A."

**Step 2 — Implement for the three standard types:**

```rust
impl<A> Functor for Option<A> {
    type Inner = A;
    type Mapped<B> = Option<B>;
    fn fmap<B>(self, f: impl FnMut(A) -> B) -> Option<B> {
        self.map(f)  // delegate to the built-in map
    }
}

impl<A, E> Functor for Result<A, E> {
    type Inner = A;
    type Mapped<B> = Result<B, E>;
    fn fmap<B>(self, f: impl FnMut(A) -> B) -> Result<B, E> {
        self.map(f)  // only transforms the Ok side
    }
}

impl<A> Functor for Vec<A> {
    type Inner = A;
    type Mapped<B> = Vec<B>;
    fn fmap<B>(self, f: impl FnMut(A) -> B) -> Vec<B> {
        self.into_iter().map(f).collect()
    }
}
```

**Step 3 — Verify Law 1 (Identity):**

```rust
fn check_identity_option<A: Clone + PartialEq>(x: Option<A>) -> bool {
    x.clone().fmap(|v| v) == x  // map with identity must return original
}

assert!(check_identity_option(Some(42)));   // true
assert!(check_identity_option::<i32>(None)); // true
```

**Step 4 — Verify Law 2 (Composition):**

```rust
fn check_composition_option<A, B, C>(
    x: Option<A>,
    f: impl Fn(A) -> B + Clone,
    g: impl Fn(B) -> C,
) -> bool
where A: Clone, B: Clone + PartialEq, C: PartialEq,
{
    let composed   = x.clone().fmap(|a| g(f(a)));  // one fmap with g∘f
    let sequential = x.fmap(f).fmap(g);             // two fmaps in sequence
    composed == sequential
}

let f = |x: i32| x * 2;
let g = |x: i32| x + 1;
assert!(check_composition_option(Some(5), f, g));  // true

// Also verify for Vec:
let xs = vec![1, 2, 3, 4, 5];
let composed:   Vec<i32> = xs.clone().fmap(|x| g(f(x)));
let sequential: Vec<i32> = xs.clone().fmap(f).fmap(g);
assert_eq!(composed, sequential);  // Law 2 holds for Vec
```

**Step 5 — What the laws *prevent*:**

If someone wrote a "Functor" that counted map calls:

```rust
// This would break Law 1:
struct CountingBox<T> { value: T, count: usize }
// After .fmap(|x| x): count would be 1, but original has count 0 → not equal
// Identity law violated → NOT a valid Functor
```

## What This Unlocks

- **Confident chaining.** You can replace `.map(f).map(g)` with `.map(|x| g(f(x)))` in any law-abiding Functor — same result, often faster (one pass instead of two).
- **Property-based testing for your own types.** The law-check functions are reusable templates. Add them to your test suite with `proptest` to verify your custom `map` implementations automatically for thousands of random inputs.
- **Generic code that works across all Functors.** Once `Option`, `Result`, and `Vec` all implement the same `Functor` trait, you can write algorithms that work over any of them — parsing pipelines, validation chains, transformation layers — without specializing for each container.

Real codebases where Functor laws matter in practice: `tokio`'s `Future::map` (async transformations must compose), `rayon`'s parallel iterators (parallel map must equal sequential map for correctness), `serde`'s value transformations, and any generic library that accepts user-provided `map`-able types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Functor interface | `module type FUNCTOR` with `type 'a t` and `val map` | `trait Functor` with GATs (`type Inner`, `type Mapped<B>`) |
| Higher-kinded types | Native — `'a option` is a type constructor | Simulated with GATs — `type Mapped<B>` approximates `F<B>` |
| Identity function | `Fun.id` from stdlib | Written as `\|v\| v` inline (no stdlib `id`) |
| Composition | `let (%) f g x = f (g x)` — explicit compose operator | Closure: `\|a\| g(f(a))` inline |
| Law verification | `assert (map Fun.id xs = xs)` | `assert_eq!(xs.clone().fmap(\|v\|v), xs)` with `Clone + PartialEq` |
| Cloning for tests | Not needed — immutable by default | Required: need original and mapped both alive for comparison |
| `Vec` equivalent | `'a list` with `List.map` | `Vec<A>` with `into_iter().map(f).collect()` |
| `Result` functor | `Result.map` (maps over `Ok` variant) | `Result::map` (same — only transforms `Ok`) |
| Compile-time law checks | Not possible — convention only | Not possible — tested at runtime with `#[test]` |
