# OCaml vs Rust: Zero-Cost Abstractions

## Side-by-Side Code

### OCaml

```ocaml
(* OCaml: filter + map + fold — creates intermediate lists *)
let sum_even_squares n =
  List.init n Fun.id
  |> List.filter (fun x -> x mod 2 = 0)
  |> List.map (fun x -> x * x)
  |> List.fold_left ( + ) 0

(* Closure-returning function — may allocate a closure record on the heap *)
let make_polynomial coeffs =
  fun x ->
    List.mapi (fun i c -> c *. (x ** float_of_int i)) coeffs
    |> List.fold_left ( +. ) 0.0
```

### Rust (idiomatic — iterator chain)

```rust
pub fn sum_even_squares(n: i64) -> i64 {
    (0..n).filter(|x| x % 2 == 0).map(|x| x * x).sum()
}
```

### Rust (functional/recursive — explicit fold)

```rust
pub fn sum_even_squares_recursive(n: i64) -> i64 {
    fn go(x: i64, limit: i64, acc: i64) -> i64 {
        if x >= limit { return acc; }
        let next = if x % 2 == 0 { acc + x * x } else { acc };
        go(x + 1, limit, next)
    }
    go(0, n, 0)
}
```

### Rust (newtype — zero-cost phantom type)

```rust
pub struct Meters(pub f64);
pub struct Seconds(pub f64);

pub fn speed(d: Meters, t: Seconds) -> f64 {
    d.0 / t.0
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Filter+map+sum | `'a list -> int` (intermediate lists) | `impl Iterator` (lazy, fused) |
| Closure-returning fn | `float list -> float -> float` (may box closure) | `fn(...) -> impl Fn(f64) -> f64` (monomorphised) |
| Newtype | `type meters = Meters of float` (constructor overhead in some backends) | `struct Meters(f64)` (transparent `repr`) |
| Polymorphic pipeline | `('a -> 'b) -> 'a list -> 'b list` | `fn<T,U,F: Fn(T)->U>(&[T], F) -> Vec<U>` |

## Key Insights

1. **Iterator fusion**: OCaml's `List.filter` and `List.map` each traverse and allocate a new list. Rust's `Iterator` adapters are lazy; the compiler fuses the entire chain into a single loop with no heap traffic.

2. **Closure monomorphisation**: OCaml closures are heap-allocated records with a function pointer (indirect call). Rust closures each get a unique anonymous struct type; at the call site the compiler sees the exact type and can inline the body completely, eliminating the indirect call and enabling further optimisation.

3. **Newtype transparency**: Both languages support newtypes for type-safe wrappers. In OCaml a constructor call may require a tag word and an allocation in boxed contexts. In Rust `#[repr(transparent)]` (the default for single-field structs) guarantees the wrapper is bit-for-bit identical to its inner type — the type information disappears entirely after type-checking.

4. **Abstraction as compiler hints**: Rust's higher-level constructs are instructions to the *compiler*, not the *CPU*. The `Iterator` protocol, `impl Trait` return types, and newtype wrappers give the optimiser the information it needs to produce the same (or better) assembly as a hand-written C loop — without asking the programmer to give up expressiveness.

5. **Verification approach**: Because Rust cannot directly expose assembly in unit tests, we verify zero-cost empirically by asserting that the iterator-chain result equals the manual-loop result for all inputs. In a release build with `cargo asm` or Godbolt, the two functions produce identical machine code.

## When to Use Each Style

**Use the iterator chain when:** you want readable, composable data transformation — `filter`, `map`, `flat_map`, `take_while`, `sum`, `fold`. The compiler handles fusion; you get clarity for free.

**Use explicit loops when:** the transformation requires mutable state that doesn't fit cleanly into a fold (e.g., sliding-window state machines), or when readability genuinely benefits from imperative style. The performance will be equivalent.

**Use newtypes when:** you want the compiler to enforce unit correctness (metres vs seconds, user IDs vs product IDs) at zero runtime cost. Prefer them over type aliases (`type Meters = f64`) which are transparent to the type checker.
