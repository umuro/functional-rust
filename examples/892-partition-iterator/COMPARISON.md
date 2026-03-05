# OCaml vs Rust: Partition Iterator

## Side-by-Side Code

### OCaml
```ocaml
(* Binary partition: evens vs odds *)
let split_even_odd lst =
  List.partition (fun x -> x mod 2 = 0) lst

(* Multi-way partition via fold_right *)
let partition3 p1 p2 lst =
  List.fold_right (fun x (a, b, c) ->
    if p1 x then (x :: a, b, c)
    else if p2 x then (a, x :: b, c)
    else (a, b, x :: c)
  ) lst ([], [], [])

(* Partition with transformation using Either *)
let partition_map f lst =
  List.fold_right (fun x (lefts, rights) ->
    match f x with
    | Either.Left l  -> (l :: lefts, rights)
    | Either.Right r -> (lefts, r :: rights)
  ) lst ([], [])
```

### Rust (idiomatic — Iterator::partition)
```rust
pub fn split_even_odd(data: &[i32]) -> (Vec<i32>, Vec<i32>) {
    data.iter().partition(|&&x| x % 2 == 0)
}
```

### Rust (functional/recursive — fold-based multi-way)
```rust
pub fn partition3<T: Clone>(
    data: &[T],
    p1: impl Fn(&T) -> bool,
    p2: impl Fn(&T) -> bool,
) -> (Vec<T>, Vec<T>, Vec<T>) {
    data.iter().cloned().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |(mut a, mut b, mut c), x| {
            if p1(&x) { a.push(x); }
            else if p2(&x) { b.push(x); }
            else { c.push(x); }
            (a, b, c)
        },
    )
}
```

### Rust (partition_map with user-defined Either)
```rust
pub fn partition_map<T, L, R>(
    data: &[T],
    f: impl Fn(&T) -> Either<L, R>,
) -> (Vec<L>, Vec<R>) {
    data.iter().fold((Vec::new(), Vec::new()), |(mut lefts, mut rights), x| {
        match f(x) {
            Either::Left(l)  => lefts.push(l),
            Either::Right(r) => rights.push(r),
        }
        (lefts, rights)
    })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Binary partition | `val partition : ('a -> bool) -> 'a list -> 'a list * 'a list` | `fn partition(self, f: impl Fn(&T) -> bool) -> (B, B)` |
| Input collection | `'a list` | `&[T]` (borrowed slice) |
| Output collection | `'a list * 'a list` | `(Vec<T>, Vec<T>)` |
| Multi-way | tuple of 3 lists | `(Vec<T>, Vec<T>, Vec<T>)` |
| Either type | `Either.Left \| Either.Right` (stdlib) | user-defined `enum Either<L, R>` |

## Key Insights

1. **Single-pass guarantee**: Both OCaml's `List.partition` and Rust's `Iterator::partition` traverse the input exactly once, routing each element to one of two output collections simultaneously — unlike two separate filter calls.

2. **Ownership vs. sharing**: OCaml lists share structure freely (persistent data); Rust must either clone (`T: Clone`) or collect references (`Vec<&T>`). Cloning here is semantically correct since both output `Vec`s own their elements independently.

3. **`Either` availability**: OCaml 4.12+ ships `Either` in the standard library. Rust's standard library does not expose `Either`, so `partition_map` requires a user-defined enum — a common pattern also addressed by the `either` crate.

4. **Multi-way extension**: OCaml's `fold_right` mirrors Rust's `.fold()`. Both accumulate a tuple of collections in a single left-to-right (or right-to-left in OCaml) scan, making the pattern directly translatable across the two languages.

5. **Generic bounds**: Rust's `Iterator::partition` is constrained to `B: Default + Extend<Self::Item>`, which `Vec<T>` satisfies out of the box. This genericity means `partition` works on any collection implementing those traits, not just `Vec`.

## When to Use Each Style

**Use `Iterator::partition` when:** splitting into exactly two groups with a simple boolean predicate and no transformation needed — it is the most concise and idiomatic choice.

**Use `fold`-based multi-way partition when:** you need three or more output buckets, or when the routing logic is complex enough that `match` branches are clearer than nested `if`/`else`.

**Use `partition_map` when:** each element must be transformed as it is routed — for example, parsing results where successes become `i32` and failures keep the original `&str`.
