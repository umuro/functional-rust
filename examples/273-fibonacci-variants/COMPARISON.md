# OCaml vs Rust: Fibonacci Variants

## Side-by-Side Code

### OCaml

```ocaml
(* Direct recursion *)
let rec fib_naive = function
  | 0 -> 0 | 1 -> 1
  | n -> fib_naive (n-1) + fib_naive (n-2)

(* Tail-recursive with accumulator *)
let fib_tail n =
  let rec go a b = function
    | 0 -> a
    | n -> go b (a + b) (n - 1)
  in go 0 1 n

(* Fold-based *)
let fib_fold n =
  let a, _ = List.init n Fun.id
    |> List.fold_left (fun (a, b) _ -> (b, a + b)) (0, 1)
  in a
```

### Rust (idiomatic — iterative loop)

```rust
pub fn fib_iter(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}
```

### Rust (functional/recursive — mirrors OCaml `fib_tail`)

```rust
pub fn fib_tail(n: u64) -> u64 {
    fn go(a: u64, b: u64, n: u64) -> u64 {
        match n {
            0 => a,
            n => go(b, a + b, n - 1),
        }
    }
    go(0, 1, n)
}
```

### Rust (fold-based — mirrors OCaml `fib_fold`)

```rust
pub fn fib_fold(n: u64) -> u64 {
    let (a, _) = (0..n).fold((0u64, 1u64), |(a, b), _| (b, a + b));
    a
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Naive recursion | `val fib_naive : int -> int` | `fn fib_naive(n: u64) -> u64` |
| Tail-recursive | `val fib_tail : int -> int` | `fn fib_tail(n: u64) -> u64` |
| Fold-based | `val fib_fold : int -> int` | `fn fib_fold(n: u64) -> u64` |
| Accumulator state | `(a, b)` threaded through `go` | `(a, b)` tuple in `.fold` closure |
| Fold input | `List.init n Fun.id` (dummy list) | `0..n` (range, zero allocation) |

## Key Insights

1. **Tail-call optimisation:** OCaml guarantees TCO; the compiler will turn `go b (a+b) (n-1)` into a jump. Rust provides no such guarantee, so the recursive `fib_tail` accumulates stack frames for large `n`. For safety, `fib_iter` is preferred in production Rust code.

2. **Fold without a list:** OCaml's `fib_fold` creates a dummy list with `List.init n Fun.id` just to have something to fold over — a common OCaml idiom. Rust avoids allocation entirely by folding over a `0..n` range iterator; the semantics are identical but the performance is better.

3. **Nested function scope:** Both OCaml's `let rec go ... in go 0 1 n` and Rust's `fn go(...) { ... }; go(0, 1, n)` use a local helper to hide the accumulator from the public API. The idiom is structurally identical across both languages.

4. **Integer type choice:** OCaml uses arbitrary-precision integers by default (in practice boxed `int`). Rust requires an explicit type: `u64` is chosen here to handle values up to `fib(93)` without overflow. For larger inputs a `u128` or `BigInt` crate would be needed.

5. **Pattern matching on integers:** OCaml `function | 0 -> ... | 1 -> ... | n -> ...` maps directly to Rust `match n { 0 => ..., 1 => ..., n => ... }`. Both bind `n` in the catch-all arm; the structural parallel is exact.

## When to Use Each Style

**Use idiomatic Rust (`fib_iter`) when:** you want safe, stack-overflow-free code for any input size with zero overhead.

**Use fold-based (`fib_fold`) when:** you are composing with other iterator adaptors or want a purely expression-oriented style without mutable variables.

**Use recursive (`fib_tail`) when:** demonstrating the OCaml accumulator pattern to learners, or when the problem structure is naturally recursive and input is bounded (e.g., ≤ 10 000).

**Avoid `fib_naive` in production:** its O(2ⁿ) time complexity makes it unusable for any n > ~35.
