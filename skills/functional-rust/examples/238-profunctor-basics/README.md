# 238: Profunctor Basics

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

The three core profunctor operations — `dimap`, `lmap`, `rmap` — plus the `Forget` and `Star` specialisations that power optic encoding.

## The Problem This Solves

You're building a processing pipeline where different parts of the codebase have different type signatures. One module gives you `String → usize`. Another needs `i32 → bool`. A third needs to plug into both.

Without a principled approach, you write one-off adapters everywhere:

```rust
fn str_len(s: String) -> usize { s.len() }

// Want: i32 -> bool (is digit count > 3?)
fn int_has_long_repr(n: i32) -> bool { str_len(n.to_string()) > 3 }

// Want: String -> String (format length)
fn str_len_formatted(s: String) -> String { format!("{} chars", str_len(s)) }

// Want: &str -> bool
fn borrow_is_long(s: &str) -> bool { str_len(s.to_string()) > 3 }
```

Each adapter is ad-hoc. There's no reusable pattern for "adapt the input" or "adapt the output" separately. When you have a dozen composable processing steps, the combinatorial explosion of adapter functions becomes unmanageable.

The profunctor abstraction names and formalises these two orthogonal adaptation directions — `lmap` for input, `rmap` for output, `dimap` for both — and extends to richer wrappers like `Forget` (read-only / always-`None` output) and `Star` (effectful output like `Vec` or `Option`). This exists to solve exactly that pain.

## The Intuition

A **profunctor** `P<A, B>` is a type that wraps some computation from `A` to `B`, with two kinds of mapping:

- **`rmap`** (right-map, covariant): adapt the *output*. Give it `B → D`, get `P<A, D>`.
- **`lmap`** (left-map, contravariant): adapt the *input*. Give it `C → A`, get `P<C, B>`.
- **`dimap`**: both at once.

The adaptation directions look like this:

```
C  ──[lmap pre]──▶  A  ──[P]──▶  B  ──[rmap post]──▶  D
   (new input)    (original)  (original)   (new output)
```

**Why is `lmap` "contravariant"?** Because to make `P<A, B>` accept `C` as input, you need `C → A` (the *opposite* direction: `C` to `A`). This is the defining characteristic of contravariance. For `rmap`, you need `B → D` (the *same* direction as output). That's covariance.

Beyond the plain function profunctor (`Mapper<A, B>`), there are specialised profunctors:

- **`Forget<A, R>`** — a profunctor that "forgets" the `B` type entirely. It runs `A → R` and ignores whatever `B` was. This is how you build *getters* in profunctor optics: `lmap` adapts the input, but `rmap` does nothing (there's no `B` to map).
- **`Star<A, B>`** — wraps `A → Vec<B>` (or `A → F<B>` for any functor `F`). `lmap` pre-composes on input; `rmap` applies inside the `Vec`. This is how you build *traversals* in profunctor optics.

**The laws** (profunctor laws) ensure these operations are coherent:
1. `dimap id id = id` — adapting with identity functions changes nothing
2. `dimap (f ∘ g) (h ∘ k) = dimap g h ∘ dimap f k` — composition is associative

## How It Works in Rust

```rust
// The canonical profunctor: wraps fn(A) -> B
pub struct Mapper<A, B> {
    f: Box<dyn Fn(A) -> B>,
}

impl<A: 'static, B: 'static> Mapper<A, B> {
    // dimap: adapt BOTH input and output
    // math: dimap pre post f = post ∘ f ∘ pre
    pub fn dimap<C: 'static, D: 'static>(
        self,
        pre:  impl Fn(C) -> A + 'static,   // adapter for input: C → A
        post: impl Fn(B) -> D + 'static,   // adapter for output: B → D
    ) -> Mapper<C, D> {
        let f = self.f;
        Mapper::new(move |c| post(f(pre(c))))
        //                    ^^^^^^^^^^^^ = post . f . pre
    }

    // lmap: adapt only input — dimap pre id
    pub fn lmap<C: 'static>(self, pre: impl Fn(C) -> A + 'static) -> Mapper<C, B> {
        let f = self.f;
        Mapper::new(move |c| f(pre(c)))
    }

    // rmap: adapt only output — dimap id post
    pub fn rmap<D: 'static>(self, post: impl Fn(B) -> D + 'static) -> Mapper<A, D> {
        let f = self.f;
        Mapper::new(move |a| post(f(a)))
    }
}

// Usage:
let str_len = Mapper::new(|s: String| s.len());

// rmap: String → usize  becomes  String → bool
let is_long = str_len.rmap(|n| n > 3);
is_long.apply("hello".to_string());  // true

// lmap: String → usize  becomes  i32 → usize  (new input type)
let num_len = Mapper::new(|s: String| s.len())
    .lmap(|n: i32| n.to_string());
num_len.apply(42);    // 2  ("42".len())
num_len.apply(1234);  // 4  ("1234".len())

// dimap: both — i32 → usize becomes i32 → usize*2
let num_doubled_len = Mapper::new(|s: String| s.len())
    .dimap(|n: i32| n.to_string(), |len| len * 2);
num_doubled_len.apply(123);  // 6  ("123".len() * 2)

// ── Forget: profunctor that ignores its output type ──────────────────

// Forget<A, R>: runs A → R, has phantom output type B (ignored)
pub struct Forget<A, R> {
    run: Box<dyn Fn(A) -> R>,
}

impl<A: 'static, R: 'static> Forget<A, R> {
    // dimap for Forget: only pre matters — the post function is ignored
    // because Forget "forgets" that B ever existed
    pub fn dimap<C: 'static, D>(
        self,
        pre:  impl Fn(C) -> A + 'static,
        _post: impl Fn(R) -> D,             // ← ignored! Forget has no B to map
    ) -> Forget<C, R> {
        let run = self.run;
        Forget::new(move |c| run(pre(c)))
    }
}

// This is how "getter" optics work:
// A getter only needs to READ — it never needs to write back.
// Forget<A, R> encodes "I can extract an R from an A, but I have no B to produce."
let get_len = Forget::new(|s: String| s.len());
let get_num_len = get_len.dimap(|n: i32| n.to_string(), |_: usize| ());
get_num_len.apply(42);  // 3 — wait, "42".len() = 2... 999 → 3

// ── Star: profunctor with effectful output ────────────────────────────

// Star<A, B>: wraps A → Vec<B>
// rmap applies inside the Vec (fmap), lmap pre-composes on input
pub struct Star<A, B> {
    run: Box<dyn Fn(A) -> Vec<B>>,
}
// This models traversals: "apply f to A, get a collection of B's"
// Used in profunctor optics for Traversal encoding
let star = Star::new(|s: String| {
    s.chars().map(|c| c as u32).collect::<Vec<_>>()   // String → Vec<char codes>
});
let star2 = star.dimap(
    |n: i32| n.to_string(),  // lmap: i32 → String
    |code| code + 1,          // rmap: increment each char code inside the Vec
);
star2.apply(42);  // char codes of "42", each + 1
```

## What This Unlocks

- **Profunctor optics foundation** — `Mapper` (functions) with `dimap` is the basis for Lens encoding; `Forget` is the basis for getters; `Star` is the basis for traversals. Understanding these three makes example 621 (profunctor optics) comprehensible.
- **Adapter composition** — instead of writing one-off adapter functions, use `lmap`/`rmap` to adapt existing transformations systematically. Useful in middleware chains, codec pipelines, and event processing.
- **Understanding variance** — `lmap` (contravariant) and `rmap` (covariant) are the two directions that type system variance refers to. If you've ever wondered why function inputs are contravariant and outputs are covariant, this is where it becomes concrete.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Profunctor type class | `module type PROFUNCTOR = sig val dimap : ('c->'a)->('b->'d)->('a,'b) p -> ('c,'d) p end` | Trait `Profunctor` — GAT limitations make full polymorphism hard |
| Function profunctor | `(->)` is a profunctor instance via type class | Explicit `Mapper<A, B>` wrapper struct |
| `Forget` | `newtype` via module / functor | `struct Forget<A, R>` — phantom `B` simply absent from fields |
| `Star` | `newtype` wrapping `Kleisli` or `a -> f b` | `struct Star<A, B>` with `Box<dyn Fn(A) -> Vec<B>>` |
| Higher-kinded types | Full HKT — `Star f a b` works for any functor `f` | Must specialise per concrete effect (`Vec`, `Option`, etc.) |
