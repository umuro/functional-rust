# Comparison: CPS — OCaml vs Rust

## Core Insight

CPS trades stack frames for heap-allocated closures. OCaml's GC makes this trade cheap — closures are first-class and lightweight. Rust's ownership model makes CPS verbose: each continuation needs `Box<dyn FnOnce>`, and nested closures fight the borrow checker. The lesson: Rust prefers explicit iteration (`for`, `fold`, explicit stack) over CPS.

## OCaml

```ocaml
let factorial_cps n =
  let rec go n k =
    if n = 0 then k 1
    else go (n - 1) (fun result -> k (n * result))
  in go n Fun.id
```

## Rust — CPS

```rust
pub fn factorial_cps(n: u64) -> u64 {
    fn go(n: u64, k: Box<dyn FnOnce(u64) -> u64>) -> u64 {
        if n == 0 { k(1) }
        else { go(n - 1, Box::new(move |result| k(n * result))) }
    }
    go(n, Box::new(|x| x))
}
```

## Rust — Idiomatic (iterative)

```rust
pub fn factorial_iter(n: u64) -> u64 {
    (1..=n).product()
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Closure cost | GC-managed, cheap | `Box<dyn FnOnce>` heap alloc |
| Identity fn | `Fun.id` | `\|x\| x` or `std::convert::identity` |
| Tail call opt | Yes (CPS enables it) | No TCO guarantee |
| Idiomatic alt | CPS is idiomatic | Iterators / explicit stack |
| Tree traversal | CPS avoids stack overflow | Explicit `Vec` stack |

## Learner Notes

- **No TCO in Rust**: Even with CPS, Rust doesn't guarantee tail call optimization, so stack overflow is still possible
- **`FnOnce` vs `Fn`**: CPS continuations are called exactly once — `FnOnce` is the right trait (takes ownership)
- **Explicit stack pattern**: `let mut stack = vec![root]; while let Some(node) = stack.pop()` — Rust's idiomatic "CPS"
- **`(1..=n).product()`**: For simple cases, iterators make CPS entirely unnecessary
- **OCaml's advantage**: CPS is a fundamental FP technique that feels natural in OCaml — it's a core teaching tool
