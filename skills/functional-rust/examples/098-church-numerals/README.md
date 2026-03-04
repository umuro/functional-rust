# 098: Church Numerals — Functions as Numbers

**Difficulty:** 3  **Level:** Advanced

Encode natural numbers as higher-order functions, then do arithmetic with those functions.

## The Problem This Solves

Every number you've ever used in Rust is ultimately stored as bits: `0u8` is eight zero bits, `1u8` is `00000001`. That's the hardware's answer to "what is a number?" But there's a deeper answer that doesn't rely on hardware at all.

In lambda calculus — the mathematical system that functional programming is built on — numbers don't exist as primitives. You have to *build* them from scratch using only functions. This forces you to answer: what is the *essence* of a number?

The Church numeral answer is beautiful: the number N is a function that applies another function N times. Three is not the bit pattern `00000011`. Three is "whatever function you give me, I will call it three times." The integer 3 is just one way to observe that behaviour — you observe it by counting how many times "add 1" gets called starting from 0.

This sounds academic but it matters: it's the mathematical proof that functions alone are Turing-complete. You don't need integers, booleans, or lists — you can build everything from functions. This example explores how far Rust lets you go down that road, and where it pushes back.

## The Intuition

Forget integers for a moment. Imagine numbers as verbs, not nouns.

- **Zero** = "do nothing to x" → return x as-is
- **One** = "do f once to x" → return f(x)
- **Two** = "do f twice to x" → return f(f(x))

```rust
// You can think of it this way (ignoring Rust types for a moment):
let zero  = |f, x| x;
let one   = |f, x| f(x);
let two   = |f, x| f(f(x));
let three = |f, x| f(f(f(x)));

// Addition: m+n = apply f m times, then n more times
// two + three: apply f 5 times total
let five = |f, x| two(f, three(f, x));

// To read the number: f = "add 1", x = 0
// five(|x| x+1, 0) → 5
```

To add m + n: apply f m times starting from wherever n leaves off. That's it.

To multiply m × n: applying n things m times = applying things m×n times.

## How It Works in Rust

Three approaches are shown, from most faithful to most practical:

**Approach A — `Box<dyn Fn>` (true Church encoding)**

```rust
// Type alias hides the complexity
type Church = Box<dyn Fn(Box<dyn Fn(i64) -> i64>) -> Box<dyn Fn(i64) -> i64>>;

fn zero() -> Church {
    Box::new(|_f| Box::new(|x| x))  // ignore f, return x unchanged
}

fn one() -> Church {
    Box::new(|f: Box<dyn Fn(i64) -> i64>| {
        Box::new(move |x| f(x))     // apply f once
    })
}

// Convert to integer: apply "add 1" to 0
fn to_int(n: &Church) -> i64 {
    let f = n(Box::new(|x| x + 1));
    f(0)
}
```

Each `Box` is a heap allocation. Compose many of these and you pay in heap allocations. OCaml does this on the stack; Rust can't because closure sizes are unknown at compile time.

**Approach B — struct wrapper (practical)**

```rust
// Store the count, apply it when needed — same semantics, no Box cascade
#[derive(Clone, Copy)]
struct ChurchNum(usize);

impl ChurchNum {
    fn apply<T>(&self, f: impl Fn(T) -> T, x: T) -> T {
        (0..self.0).fold(x, |acc, _| f(acc))
        // fold applies f self.0 times — the Church numeral's actual behaviour
    }

    fn add(self, other: Self) -> Self { ChurchNum(self.0 + other.0) }
    fn mul(self, other: Self) -> Self { ChurchNum(self.0 * other.0) }
}
```

**Approach C — generic function**

```rust
// Direct Church application without wrapping
fn church_apply<T>(n: usize, f: impl Fn(T) -> T, x: T) -> T {
    (0..n).fold(x, |acc, _| f(acc))
}

// Church encoding of 3 applied to doubling starting from 1:
// 1 → 2 → 4 → 8
assert_eq!(church_apply(3, |x: i32| x * 2, 1), 8);
```

## What This Unlocks

- **Lambda calculus comprehension**: You can now read foundational CS texts that encode everything as functions. This shows up in type theory, proof assistants (Coq, Agda), and Haskell.
- **Seeing through abstraction layers**: Understanding that numbers are just "apply-N-times functions" explains why iterators in Rust, `fold`, and function composition are so powerful — they're the same underlying idea.
- **Expressive trait design**: The `apply` method on `ChurchNum` generalizes to any monoidal structure. This pattern appears in parser combinators, retry logic, and middleware chains.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type of Church numeral | `('a -> 'a) -> 'a -> 'a` (polymorphic, zero cost) | `Box<dyn Fn(...)>` (heap, erased type) |
| Polymorphism | Implicit, free | Must use generics or `dyn Fn` |
| Closure allocation | Stack/GC | Heap via `Box` |
| Composition cost | Free | Each composition = heap alloc |
| Practical recommendation | Use directly | Use struct wrapper |
| Successor function | `let succ n f x = f (n f x)` | 10+ lines with `Rc` for sharing |
