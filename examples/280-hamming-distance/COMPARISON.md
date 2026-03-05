# OCaml vs Rust: Hamming Distance

## Side-by-Side Code

### OCaml
```ocaml
(* Imperative version *)
let hamming s1 s2 =
  if String.length s1 <> String.length s2 then
    Error "strands must be of equal length"
  else
    let dist = ref 0 in
    String.iteri (fun i c ->
      if c <> s2.[i] then incr dist
    ) s1;
    Ok !dist

(* Pure functional version *)
let hamming_fp s1 s2 =
  if String.length s1 <> String.length s2 then Error "unequal"
  else
    Ok (Seq.zip (String.to_seq s1) (String.to_seq s2)
    |> Seq.fold_left (fun acc (a, b) -> if a <> b then acc + 1 else acc) 0)
```

### Rust (idiomatic)
```rust
pub fn hamming(s1: &str, s2: &str) -> Result<usize, &'static str> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length");
    }
    Ok(s1.chars().zip(s2.chars()).filter(|(a, b)| a != b).count())
}
```

### Rust (functional/fold)
```rust
pub fn hamming_fold(s1: &str, s2: &str) -> Result<usize, &'static str> {
    if s1.len() != s2.len() {
        return Err("strands must be of equal length");
    }
    Ok(s1.chars().zip(s2.chars())
        .fold(0, |acc, (a, b)| if a != b { acc + 1 } else { acc }))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val hamming : string -> string -> (int, string) result` | `fn hamming(s1: &str, s2: &str) -> Result<usize, &'static str>` |
| Success | `Ok 7` | `Ok(7)` |
| Error | `Error "message"` | `Err("message")` |
| Mutable counter | `let dist = ref 0` ... `incr dist` ... `!dist` | `let mut dist = 0` ... `dist += 1` |
| Char comparison | `c <> s2.[i]` (index access) | `a != b` (via zip, no indexing) |

## Key Insights

1. **Result type is almost identical:** OCaml's `(int, string) result` with `Ok/Error` maps directly to Rust's `Result<usize, &str>` with `Ok/Err` — the error handling pattern is the same in both languages
2. **Zip eliminates indexing:** OCaml's imperative version uses `String.iteri` with index `i` and `s2.[i]`; Rust's zip pairs characters automatically — no index needed, no bounds-check concerns
3. **Filter+count > fold for counting:** Rust's `.filter(pred).count()` is more declarative than `.fold(0, |acc, x| if pred(x) { acc + 1 } else { acc })` — it communicates "count matching items" directly
4. **`ref` vs `mut`:** OCaml's `ref` creates a heap-allocated mutable cell; Rust's `mut` is a stack variable — both allow mutation in an otherwise functional style, but Rust's is zero-cost
5. **String length check:** Both languages check length upfront and return early on mismatch. OCaml's `String.length` and Rust's `str::len()` are both O(1) for byte length (but note: Rust's `.chars().count()` would be O(n) for char count)

## When to Use Each Style

**Use idiomatic Rust when:** You want the clearest, most concise code — `zip().filter().count()` is a single pipeline that reads like English: "zip the characters, filter where they differ, count the differences."

**Use fold Rust when:** You need to accumulate something more complex than a count — fold generalizes to any accumulator. Here it's overkill, but the pattern is worth knowing for more complex reductions.
