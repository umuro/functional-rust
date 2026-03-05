# OCaml vs Rust: Series — Sliding Window

## Side-by-Side Code

### OCaml
```ocaml
let series n s =
  if n > String.length s then []
  else
    List.init (String.length s - n + 1) (fun i ->
      String.sub s i n
    )

let largest_product n s =
  if n = 0 then Ok 1
  else if n > String.length s then Error "span too large"
  else
    series n s
    |> List.map (fun sub ->
      String.fold_left (fun acc c ->
        acc * (Char.code c - Char.code '0')
      ) 1 sub
    )
    |> List.fold_left max 0
    |> fun m -> Ok m
```

### Rust (idiomatic)
```rust
pub fn series(n: usize, s: &str) -> Vec<String> {
    if n == 0 { return vec![String::new(); s.len() + 1]; }
    s.as_bytes()
        .windows(n)
        .map(|w| std::str::from_utf8(w).unwrap().to_owned())
        .collect()
}

pub fn largest_product(n: usize, s: &str) -> Result<u64, String> {
    if n == 0 { return Ok(1); }
    if n > s.len() { return Err("span too large".to_string()); }
    let max = series(n, s)
        .into_iter()
        .map(|sub| sub.chars().map(|c| c as u64 - '0' as u64).product::<u64>())
        .max()
        .unwrap_or(0);
    Ok(max)
}
```

### Rust (functional/recursive)
```rust
pub fn largest_product_recursive(n: usize, s: &str) -> Result<u64, String> {
    if n == 0 { return Ok(1); }
    if n > s.len() { return Err("span too large".to_string()); }
    fn digit_product(s: &str) -> u64 {
        s.chars().map(|c| c as u64 - '0' as u64).product()
    }
    fn go(n: usize, s: &str, best: u64) -> u64 {
        if s.len() < n { best }
        else {
            let p = digit_product(&s[..n]);
            go(n, &s[1..], best.max(p))
        }
    }
    Ok(go(n, s, 0))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Sliding windows | `val series : int -> string -> string list` | `fn series(n: usize, s: &str) -> Vec<String>` |
| Product with error | `val largest_product : int -> string -> (int, string) result` | `fn largest_product(n: usize, s: &str) -> Result<u64, String>` |
| Fold over chars | `String.fold_left : ('a -> char -> 'a) -> 'a -> string -> 'a` | `.chars().fold(init, f)` or `.product()` |
| Max of list | `List.fold_left max 0` | `.max().unwrap_or(0)` |
| Optional integer | `'a option` | `Option<T>` |

## Key Insights

1. **`slice::windows()` is the idiomatic Rust primitive.** OCaml has no built-in sliding-window for strings; you reconstruct it with `List.init` + `String.sub`. Rust's `slice::windows(n)` is provided by the standard library and produces borrowed sub-slices with zero additional allocation, making it both ergonomic and efficient.

2. **Byte slice vs. character slice.** Rust strings are UTF-8, so `&str` has no `O(1)` indexing. Working via `as_bytes()` is safe here because ASCII digit strings are single-byte, and `from_utf8` re-borrows the window as `&str` without copying.

3. **`product()` replaces `fold_left (*) 1`.** OCaml's `String.fold_left (fun acc c -> acc * digit_of c) 1 sub` maps directly to `.chars().map(digit_of).product::<u64>()` in Rust. The `product` iterator method is the exact Rust idiom for multiplicative folds.

4. **Explicit integer width matters.** OCaml integers are arbitrary precision (via the runtime); Rust requires you to pick a type. `u64` safely holds the product of up to 19 single-digit values before overflow, which covers all practical inputs for this problem.

5. **Recursive tail call with accumulator.** The recursive Rust version passes a `best` accumulator through `go(n, &s[1..], best.max(p))`, consuming one character per recursive call — exactly the OCaml `List.fold_left` pattern translated to explicit recursion on string slices.

## When to Use Each Style

**Use idiomatic Rust when:** you want readable, allocation-light code that composes naturally with the rest of the iterator pipeline. `windows()` + `map()` + `max()` reads as a single declarative query.

**Use recursive Rust when:** teaching the OCaml parallel explicitly, or when the algorithm is naturally expressed as "process head, recurse on tail" and you want to avoid collecting an intermediate `Vec`.
