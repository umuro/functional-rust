# 057: Monad Laws

**Difficulty:** ⭐⭐  **Level:** Intermediate

Three rules that guarantee a monad's chain behaves predictably — and how to verify them in code.

## The Problem This Solves

Imagine you're using a library that claims its type supports `and_then`-style chaining. You write a multi-step pipeline. It seems to work on your test cases. Then you refactor — you restructure which steps happen in which order, or you add a no-op step to debug something — and suddenly the results change even though the logic shouldn't have.

The problem: the library's "monad" isn't really a monad. Its `bind`/`and_then` implementation violates one of three invariants that every real monad must satisfy. Without those invariants, refactoring monadic chains isn't safe. You can't trust that reorganizing steps leaves behavior unchanged.

This is subtle and real. A custom wrapper type that adds logging, caching, or other effects alongside chaining can accidentally break these invariants. Your `Option` and `Result` are safe because they're battle-tested. But if you ever implement your own monad-like type, you need to verify these laws.

The three monad laws exist to solve exactly that pain: they're the minimal contract that makes monadic chaining trustworthy.

## The Intuition

Think of the laws as the rules that make function composition sane. When you write `f(g(x))`, you expect: if you replace `g` with a function that just returns its input unchanged, nothing changes. If you add parentheses differently, nothing changes. These feel obvious for regular functions.

Monads have the same expectations, stated for chaining with `and_then`:

**Law 1: Left Identity** — Wrapping a value and immediately binding a function to it is the same as just calling that function directly.

```rust
// These must be equal for any value `a` and function `f`:
Some(a).and_then(f)  ==  f(a)
// "Wrapping in Some and immediately unwrapping via and_then does nothing"
```

**Law 2: Right Identity** — Binding `Some` (the wrapping function) to a monad gives back the original monad.

```rust
// These must be equal for any Option `m`:
m.and_then(Some)  ==  m
// "Binding with the identity wrapper does nothing"
```

**Law 3: Associativity** — It doesn't matter how you group the chain. Two steps then a third equals one step then two steps.

```rust
// These must be equal:
m.and_then(f).and_then(g)  ==  m.and_then(|x| f(x).and_then(g))
// "Nesting the chain differently doesn't change the result"
```

Law 3 is the crucial one for refactoring: it means you can freely restructure a chain into sub-functions without changing behavior.

## How It Works in Rust

**Verification functions for Option**

```rust
fn double(x: i32) -> Option<i32> { Some(x * 2) }
fn inc(x: i32) -> Option<i32>    { Some(x + 1) }

// Law 1: Left Identity
// Some(a).and_then(f) == f(a)
fn verify_left_identity<A: Clone, B: PartialEq>(a: A, f: fn(A) -> Option<B>) -> bool {
    Some(a.clone()).and_then(f) == f(a)
}

// Law 2: Right Identity
// m.and_then(Some) == m
fn verify_right_identity<A: Clone + PartialEq>(m: Option<A>) -> bool {
    m.clone().and_then(Some) == m
}

// Law 3: Associativity
// m.and_then(f).and_then(g) == m.and_then(|x| f(x).and_then(g))
fn verify_associativity<A: Clone + PartialEq>(
    m: Option<A>, f: fn(A) -> Option<A>, g: fn(A) -> Option<A>
) -> bool {
    let left  = m.clone().and_then(f).and_then(g);
    let right = m.and_then(|x| f(x).and_then(g));
    left == right
}
```

**Running the checks**

```rust
// Law 1: Some(5).and_then(double) == double(5) == Some(10)
assert!(verify_left_identity(5, double));   // true

// Law 2: Some(42).and_then(Some) == Some(42)
assert!(verify_right_identity(Some(42)));   // true
assert!(verify_right_identity(None::<i32>)); // true — None stays None

// Law 3: (Some(5) >>= double >>= inc) == (Some(5) >>= (double then inc))
assert!(verify_associativity(Some(5), double, inc));  // true
```

**Laws hold for Result too**

```rust
fn verify_result_left_identity<A: Clone, B: PartialEq>(
    a: A, f: fn(A) -> Result<B, String>
) -> bool {
    Ok::<A, String>(a.clone()).and_then(f) == f(a)
}

fn verify_result_right_identity<A: Clone + PartialEq>(m: Result<A, String>) -> bool {
    m.clone().and_then(Ok) == m
}

// Errors flow through correctly
assert!(verify_result_right_identity(Ok::<i32, String>(42)));
assert!(verify_result_right_identity(Err::<i32, String>("oops".into())));
```

**Vec as a monad (multiple results)**

`Vec` is also a monad — `bind` becomes "apply to each element, flatten results":

```rust
fn vec_bind<A, B>(xs: Vec<A>, f: fn(&A) -> Vec<B>) -> Vec<B> {
    xs.iter().flat_map(f).collect()
}

// Left identity: vec_bind(vec![a], f) == f(&a)
fn verify_vec_left_identity<A: Clone + PartialEq, B: PartialEq>(
    a: A, f: fn(&A) -> Vec<B>
) -> bool {
    vec_bind(vec![a.clone()], f) == f(&a)
}
```

Note: the laws hold for `None`, `Err`, and empty `Vec` — all the "empty" cases. This is important: a monad's laws must hold across all values, not just the happy path.

## What This Unlocks

- **Safe refactoring of chains.** When you know your type satisfies the laws, you can split a long chain into sub-functions, extract common sub-chains, or reorder grouping — and be certain the behavior is unchanged.
- **Confidence when implementing custom types.** Writing a wrapper type that supports `and_then`? Run these three checks. If they pass, your implementation is correct. If they fail, callers will hit subtle bugs when they refactor.
- **Understanding why `flat_map` is everywhere.** `Iterator::flat_map`, `Option::and_then`, `Result::and_then`, `Future::and_then` — these all obey the same three laws. The laws unify them conceptually, which is why people call them all "monadic bind."

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bind operator | `>>=` (custom infix) | `and_then` method |
| Law verification | Structural equality with `=` | `assert_eq!` with `PartialEq` derive |
| Cloning for tests | Not needed (immutable GC values) | Requires `.clone()` to test both sides |
| List monad bind | `List.concat_map f xs` | `xs.iter().flat_map(f).collect()` |
| Law enforcement | Convention only — compiler won't check | Convention only — compiler won't check |
