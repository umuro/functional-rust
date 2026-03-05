# OCaml vs Rust: Seq — Zip and Iterate

## Side-by-Side Code

### OCaml

```ocaml
(* Seq.zip — pair two sequences element-by-element *)
let letters = List.to_seq ['a'; 'b'; 'c'; 'd']
let numbers = List.to_seq [1; 2; 3; 4]
let pairs = Seq.zip letters numbers |> List.of_seq

(* Seq.iterate — repeated function application *)
let collatz n = if n mod 2 = 0 then n / 2 else 3 * n + 1
let seq = Seq.iterate collatz 27 |> Seq.take 20 |> List.of_seq
```

### Rust (idiomatic — `Iterator::zip` + `std::iter::successors`)

```rust
pub fn zip_slices<A: Copy, B: Copy>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter().copied().zip(b.iter().copied()).collect()
}

pub fn collatz(n: u64) -> u64 {
    if n.is_multiple_of(2) { n / 2 } else { 3 * n + 1 }
}

pub fn iterate<T: Clone, F: Fn(&T) -> T>(f: F, start: T, n: usize) -> Vec<T> {
    std::iter::successors(Some(start), |prev| Some(f(prev)))
        .take(n)
        .collect()
}
```

### Rust (functional / recursive — explicit accumulator)

```rust
pub fn iterate_recursive<T: Clone, F: Fn(&T) -> T>(f: &F, start: T, n: usize) -> Vec<T> {
    fn go<T: Clone, F: Fn(&T) -> T>(f: &F, current: T, remaining: usize, acc: &mut Vec<T>) {
        if remaining == 0 { return; }
        let next = f(&current);
        acc.push(current);
        go(f, next, remaining - 1, acc);
    }
    let mut acc = Vec::with_capacity(n);
    go(f, start, n, &mut acc);
    acc
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Zip two sequences | `Seq.zip : 'a Seq.t -> 'b Seq.t -> ('a * 'b) Seq.t` | `Iterator::zip` → `Zip<A, B>` |
| Repeated application | `Seq.iterate : ('a -> 'a) -> 'a -> 'a Seq.t` | `successors(Some(x), \|p\| Some(f(p)))` |
| Take prefix | `Seq.take : int -> 'a Seq.t -> 'a Seq.t` | `.take(n)` |
| Materialise | `List.of_seq : 'a Seq.t -> 'a list` | `.collect::<Vec<_>>()` |
| Collatz step | `val collatz : int -> int` | `fn collatz(n: u64) -> u64` |

## Key Insights

1. **Direct structural equivalence:** `Seq.iterate f x |> Seq.take n |> List.of_seq`
   maps one-to-one to `successors(Some(x), |p| Some(f(p))).take(n).collect()`.
   The pipeline operators `|>` become method chains `.`.

2. **Zero-cost vs heap-allocated laziness:** OCaml sequences are closures stored
   on the heap; Rust iterators are state machines whose state lives on the stack
   and whose dispatch is monomorphised at compile time — no allocation, no
   virtual dispatch.

3. **`&T` in Rust closures vs `T` in OCaml:** `Seq.iterate` passes the current
   value directly to `f`. Rust's `successors` passes `&T` to avoid a move,
   so the closure must produce a new owned `T`. This requires `T: Clone` when
   the value is not `Copy`, and it's where the ownership model surfaces.

4. **`successors` is strictly more general than `Seq.iterate`:** Returning `None`
   from the `successors` closure terminates the iterator — OCaml has no equivalent
   within `Seq.iterate` itself (you'd need `Seq.unfold`).

5. **`Seq.zip` length semantics are identical:** Both OCaml and Rust stop at the
   shorter sequence. This is the natural choice for lazy sequences where one may
   be infinite.

## When to Use Each Style

**Use idiomatic Rust (`successors` + method chains) when:** building a processing
pipeline, because the chain reads top-to-bottom like the OCaml `|>` pipeline and
the compiler inlines everything.

**Use recursive Rust when:** teaching the OCaml structural recursion pattern
explicitly, or when you need fine-grained control over the accumulation order.
