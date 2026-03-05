# OCaml vs Rust: Seq.map, Seq.filter — Lazy Transformations

## Side-by-Side Code

### OCaml

```ocaml
(* Infinite sequence of naturals via Seq.unfold *)
let naturals = Seq.unfold (fun n -> Some (n, n + 1)) 1

(* Lazy pipeline: square → keep evens → take 8 → force to list *)
let even_squares =
  naturals
  |> Seq.map (fun n -> n * n)
  |> Seq.filter (fun n -> n mod 2 = 0)
  |> Seq.take 8
  |> List.of_seq
(* [4; 16; 36; 64; 100; 144; 196; 256] *)
```

### Rust (idiomatic)

```rust
// Infinite range + iterator adapters — structurally identical to the OCaml pipeline
pub fn even_squares_idiomatic(k: usize) -> Vec<u64> {
    (1u64..)
        .map(|n| n * n)
        .filter(|n| n % 2 == 0)
        .take(k)
        .collect()
}
```

### Rust (functional / successors)

```rust
// std::iter::successors mirrors Seq.unfold: explicit seed + step function
pub fn even_squares_with_successors(k: usize) -> Vec<u64> {
    std::iter::successors(Some(1u64), |&n| Some(n + 1))
        .map(|n| n * n)
        .filter(|n| n % 2 == 0)
        .take(k)
        .collect()
}
```

### Rust (generic map-then-filter)

```rust
// Encodes `Seq.map f |> Seq.filter p` as a reusable generic function
pub fn map_then_filter<I, T, U, F, P>(iter: I, f: F, p: P) -> Vec<U>
where
    I: Iterator<Item = T>,
    F: Fn(T) -> U,
    P: Fn(&U) -> bool,
{
    iter.map(f).filter(|u| p(u)).collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Infinite sequence | `int Seq.t` | `impl Iterator<Item = u64>` |
| `Seq.unfold` | `('b -> ('a * 'b) option) -> 'b -> 'a Seq.t` | `std::iter::successors(Some(seed), step)` |
| `Seq.map` | `('a -> 'b) -> 'a Seq.t -> 'b Seq.t` | `.map(\|x\| f(x))` adapter |
| `Seq.filter` | `('a -> bool) -> 'a Seq.t -> 'a Seq.t` | `.filter(\|x\| p(x))` adapter |
| `Seq.take` | `int -> 'a Seq.t -> 'a Seq.t` | `.take(k)` adapter |
| Force to list | `List.of_seq` | `.collect::<Vec<_>>()` |

## Key Insights

1. **Structural equivalence:** The Rust iterator chain `(1u64..).map(sq).filter(even).take(k).collect()` is line-for-line identical in structure to the OCaml `Seq.map sq |> Seq.filter even |> Seq.take k |> List.of_seq`. The pipeline shape is preserved.

2. **Zero heap cost for adapters:** OCaml thunks are heap-allocated closures (GC'd). Rust iterator adapters are plain structs on the stack — the compiler monomorphises and inlines them completely. There is no iterator overhead at runtime.

3. **Infinite sequences without magic:** OCaml's `Seq.unfold` and Rust's `std::iter::successors` are symmetric: both take a seed value and a step function returning an `Option` of `(value, next_seed)`. The `None` case terminates the sequence; returning `Some` forever gives an infinite sequence.

4. **Forcing evaluation:** In OCaml, `List.of_seq` (or `Seq.take n |> List.of_seq`) is the consumer that drives the thunks. In Rust, `.collect()` is the consumer that drives the iterator state machine. Nothing is computed in either language until the consumer demands it.

5. **Type inference and overflow safety:** OCaml uses arbitrary-precision integers by default (`int` is 63-bit); squaring large naturals is safe. In Rust you must choose `u64` (or `u128`) explicitly to avoid wrapping on large inputs — the type forces you to think about overflow at the API boundary.

## When to Use Each Style

**Use idiomatic Rust (infinite range) when:** you need a simple arithmetic progression and readability matters — `(1u64..)` communicates "all naturals from 1" instantly.

**Use `successors` when:** the step function is more complex than +1, or you want to mirror OCaml's `Seq.unfold` structure explicitly for pedagogical clarity or when the next state depends on the previous state in a non-trivial way.
