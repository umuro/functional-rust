**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[series-sliding-window on hightechmind.io](https://hightechmind.io/posts/functional-rust/series-sliding-window)

---

## Problem Statement

Extract all contiguous substrings of length `n` from a string using a sliding window. Apply this to find the window with the largest digit product. Implement two variants: the byte-slice `.windows(n)` approach and an explicit index range version. Also implement a recursive variant that mirrors OCaml's list-based sliding window style.

## Learning Outcomes

- Use `.as_bytes().windows(n)` to produce zero-copy overlapping byte windows over a string slice
- Convert byte windows to substrings via `std::str::from_utf8`
- Implement the index-range variant: `(0..=len - n).map(|i| &s[i..i+n])`
- Compute the largest digit product by chaining `.windows()` → `.map(digit_product)` → `.max()`
- Validate inputs with `Result`: span too large, invalid (non-digit) characters

## Rust Application

```rust
pub fn series(n: usize, s: &str) -> Vec<String> {
    if n == 0 {
        return vec![String::new(); s.len() + 1];
    }
    s.as_bytes()
        .windows(n)
        .map(|w| std::str::from_utf8(w).unwrap().to_owned())
        .collect()
}

pub fn largest_product(n: usize, s: &str) -> Result<u64, String> {
    if n == 0 { return Ok(1); }
    if n > s.len() { return Err("span too large".to_string()); }
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return Err("invalid character".to_string());
    }
    let max = series(n, s)
        .into_iter()
        .map(|sub| sub.chars().map(|c| c as u64 - '0' as u64).product::<u64>())
        .max()
        .unwrap_or(0);
    Ok(max)
}
```

`.windows(n)` on a byte slice produces overlapping `&[u8]` windows of exactly `n` bytes, advancing by one byte per step. For ASCII strings this is equivalent to character windows. The function returns `n - k + 1` windows for a string of length `k`.

The edge case `n == 0` requires special handling: the mathematical convention is that an empty window product is 1, and there are `len + 1` empty substrings of a string of length `len`.

## OCaml Approach

```ocaml
let series n s =
  let len = String.length s in
  if n = 0 then List.init (len + 1) (fun _ -> "")
  else List.init (len - n + 1) (fun i -> String.sub s i n)

let digit_product sub =
  String.fold_left (fun acc c -> acc * (Char.code c - Char.code '0')) 1 sub

let largest_product n s =
  if n = 0 then Ok 1
  else if n > String.length s then Error "span too large"
  else
    series n s
    |> List.map digit_product
    |> List.fold_left max 0
    |> Result.ok
```

OCaml's `List.init k f` generates a list of `k` elements where element `i` is `f(i)`. This creates substrings without mutation. `String.fold_left` computes the digit product purely functionally.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Window source | `.as_bytes().windows(n)` — zero-copy | `String.sub` — allocates each substring |
| Index iteration | `(0..=len-n).map(\|i\| ...)` | `List.init (len-n+1) (fun i -> ...)` |
| Error handling | `Result<u64, String>` with `?` | `result` type |
| Product | `.product::<u64>()` | `List.fold_left ( * ) 1` |

`.windows(n)` is a zero-copy operation — it hands out references into the original byte slice. Converting to `String` allocates, but the window itself does not. For very large inputs, processing windows without collecting saves memory.

## Exercises

1. Implement `max_window_sum(n, nums: &[i64])` — the maximum sum of any contiguous subarray of length `n`.
2. Implement `distinct_windows(n, s)` — count the number of distinct substrings of length `n`.
3. Add validation that the span is non-zero in `series` and return an empty `Vec` rather than the current `n+1` empty strings.
4. Implement a streaming version that processes windows one at a time without collecting the full `Vec<String>`.
5. Extend `largest_product` to return the starting index of the maximum window in addition to the product value.
