# OCaml vs Rust: Numeric Reductions — sum() and product()

## Side-by-Side Code

### OCaml
```ocaml
let sum lst = List.fold_left (+) 0 lst
let product lst = List.fold_left ( * ) 1 lst

let factorial n = product (List.init n (fun i -> i + 1))
let sum_squares nums = sum (List.map (fun x -> x * x) nums)
```

### Rust (idiomatic)
```rust
fn sum_ints(nums: &[i32]) -> i32      { nums.iter().copied().sum() }
fn product_ints(nums: &[i32]) -> i32  { nums.iter().copied().product() }
fn factorial(n: u64) -> u64           { (1..=n).product() }
fn sum_of_squares(nums: &[i32]) -> i32 { nums.iter().map(|&x| x * x).sum() }
```

### Rust (explicit fold — shows the identity)
```rust
fn sum_fold(nums: &[i32]) -> i32 {
    nums.iter().copied().fold(0, |acc, x| acc + x)
}

fn product_fold(nums: &[i32]) -> i32 {
    nums.iter().copied().fold(1, |acc, x| acc * x)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Sum function | `val sum : int list -> int` | `fn sum_ints(nums: &[i32]) -> i32` |
| Product function | `val product : int list -> int` | `fn product_ints(nums: &[i32]) -> i32` |
| Fold with identity | `List.fold_left (+) 0 lst` | `iter.fold(0, \|acc, x\| acc + x)` |
| Idiomatic shorthand | — | `.sum()` / `.product()` |
| Generic bound | polymorphic `+` / `*` | `T: Sum` / `T: Product` |

## Key Insights

1. **Named intent:** OCaml requires spelling out `List.fold_left (+) 0` every time; Rust's `.sum()` and `.product()` name the operation and hide the identity element, making call-sites self-documenting.
2. **Zero-cost abstraction:** Both `sum()` and `product()` compile down to the same machine code as the explicit `fold` — the abstraction is pure syntax sugar with no runtime overhead.
3. **Trait-driven generics:** Rust uses the `Sum` and `Product` traits, so the same `.sum()` call works on `i32`, `f64`, `u64`, and even `Option<T>` — the type is resolved at compile time with no dynamic dispatch.
4. **Ranges as iterators:** Rust ranges (`1..=n`) implement `Iterator`, so `(1..=n).product()` expresses factorial without building an intermediate list — OCaml needs `List.init` to create that list first.
5. **Empty-collection identity:** Both languages agree on the mathematical identity: `sum([]) = 0`, `product([]) = 1`. Rust's trait implementations encode this contract; OCaml's `fold_left` makes it the caller's responsibility to pass the right seed.

## When to Use Each Style

**Use `.sum()` / `.product()` when:** you want readable, intent-revealing code and the operation is a straightforward numeric reduction — the common case.
**Use `.fold()` when:** the identity or combining function is non-standard (e.g., `fold(f64::NEG_INFINITY, f64::max)` for max) or when you need to carry extra state alongside the accumulator.
