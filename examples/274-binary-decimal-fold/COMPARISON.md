# OCaml vs Rust: Binary ↔ Decimal Fold

## Side-by-Side Code

### OCaml
```ocaml
let binary_to_decimal s =
  String.fold_left (fun acc c ->
    match c with
    | '0' -> acc * 2
    | '1' -> acc * 2 + 1
    | _ -> failwith "invalid binary digit"
  ) 0 s

let decimal_to_binary n =
  if n = 0 then "0"
  else
    let rec go n acc =
      if n = 0 then acc
      else go (n / 2) (string_of_int (n mod 2) ^ acc)
    in go n ""
```

### Rust (idiomatic — try_fold)
```rust
pub fn binary_to_decimal(s: &str) -> Result<u64, String> {
    s.chars().try_fold(0u64, |acc, c| match c {
        '0' => Ok(acc * 2),
        '1' => Ok(acc * 2 + 1),
        _ => Err(format!("invalid binary digit: {c}")),
    })
}
```

### Rust (functional/recursive decimal_to_binary)
```rust
pub fn decimal_to_binary(n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    fn go(n: u64, acc: String) -> String {
        if n == 0 {
            acc
        } else {
            go(n / 2, format!("{}{}", n % 2, acc))
        }
    }
    go(n, String::new())
}
```

### Rust (iterative decimal_to_binary)
```rust
pub fn decimal_to_binary_iter(n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut bits = Vec::new();
    let mut x = n;
    while x > 0 {
        bits.push((x % 2) as u8);
        x /= 2;
    }
    bits.iter().rev().map(|b| b.to_string()).collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| binary→decimal | `val binary_to_decimal : string -> int` | `fn binary_to_decimal(s: &str) -> Result<u64, String>` |
| decimal→binary | `val decimal_to_binary : int -> string` | `fn decimal_to_binary(n: u64) -> String` |
| Error reporting | raises `Failure "invalid binary digit"` | returns `Err(String)` |
| Fold | `String.fold_left f init s` | `s.chars().try_fold(init, f)` |
| Accumulator | `int` (unbounded) | `u64` (64-bit unsigned) |

## Key Insights

1. **`try_fold` is `fold_left` with a short-circuit:** OCaml's `fold_left` continues regardless of intermediate state — if you want to abort on invalid input you need `failwith`. Rust's `try_fold` short-circuits the moment a closure returns `Err`, making error-aware folds first-class.

2. **No exceptions — `Result` at the boundary:** OCaml raises an exception that unwinds the call stack; callers must use `try…with` defensively. Rust returns `Result<u64, String>`, forcing callers to handle the error case explicitly — safer and more composable.

3. **Inner functions for recursion:** OCaml's `let rec go … in go n ""` pattern translates naturally to a Rust inner `fn go` inside the outer function. Rust inner functions are not closures — they cannot capture the enclosing scope — which matches the OCaml style exactly.

4. **String accumulation strategy:** OCaml uses `^` to prepend the new digit to the accumulator string, building the result right-to-left naturally. Rust's `format!("{}{}", n % 2, acc)` mirrors this. The iterative approach instead appends to a `Vec` and reverses — O(n) either way but avoids repeated string allocations.

5. **Type precision:** OCaml's `int` is platform-width and signed, meaning `decimal_to_binary` accepts negative numbers silently. Rust's `u64` makes the domain explicit — negative inputs are a compile-time type error, not a runtime surprise.

## When to Use Each Style

**Use `try_fold` (idiomatic Rust):** Any time you fold over an iterator and individual steps may fail. It is the standard Rust idiom for validated accumulation and composes cleanly with `?`.

**Use recursive `go` helper:** When you want the code to read closest to the OCaml original, or when the problem is naturally expressed as a decreasing recursion with an accumulator. Good for teaching the OCaml→Rust translation.

**Use iterative `decimal_to_binary_iter`:** When performance matters and you want to avoid stack growth from deep recursion on large numbers. Collecting into a `Vec` then reversing is idiomatic Rust and avoids repeated string re-allocation.
