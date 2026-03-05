# OCaml vs Rust: Atbash Cipher

## Side-by-Side Code

### OCaml

```ocaml
let atbash_char c =
  if c >= 'a' && c <= 'z' then
    Some (Char.chr (Char.code 'z' - (Char.code c - Char.code 'a')))
  else if c >= '0' && c <= '9' then Some c
  else None

let encode s =
  let chars = String.to_seq (String.lowercase_ascii s)
    |> Seq.filter_map atbash_char
    |> List.of_seq in
  let rec group = function
    | [] -> []
    | cs ->
      let chunk = List.filteri (fun j _ -> j < 5) cs in
      let rest  = List.filteri (fun j _ -> j >= 5) cs in
      String.init (List.length chunk) (List.nth chunk)
      :: group rest
  in
  String.concat " " (group chars)
```

### Rust (idiomatic)

```rust
fn atbash_char(c: char) -> Option<char> {
    if c.is_ascii_lowercase() {
        Some((b'z' - (c as u8 - b'a')) as char)
    } else if c.is_ascii_digit() {
        Some(c)
    } else {
        None
    }
}

pub fn encode(input: &str) -> String {
    let chars: Vec<char> = input
        .to_lowercase()
        .chars()
        .filter_map(atbash_char)
        .collect();

    chars
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}
```

### Rust (functional/recursive — mirrors OCaml grouping)

```rust
pub fn encode_recursive(input: &str) -> String {
    let chars: Vec<char> = input
        .to_lowercase()
        .chars()
        .filter_map(atbash_char)
        .collect();

    fn group(chars: &[char]) -> Vec<String> {
        if chars.is_empty() {
            return vec![];
        }
        let (chunk, rest) = chars.split_at(chars.len().min(5));
        let mut result = vec![chunk.iter().collect::<String>()];
        result.extend(group(rest));
        result
    }

    group(&chars).join(" ")
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Character map | `val atbash_char : char -> char option` | `fn atbash_char(c: char) -> Option<char>` |
| Encode | `val encode : string -> string` | `fn encode(input: &str) -> String` |
| Decode | `val decode : string -> string` | `fn decode(input: &str) -> String` |
| Optional char | `char option` | `Option<char>` |
| Character sequence | `char Seq.t` | `impl Iterator<Item = char>` |

## Key Insights

1. **`chunks` vs `filteri`:** OCaml has no direct `List.chunks` so the code uses `List.filteri` with index predicates (`j < 5`, `j >= 5`) to emulate it — O(n²) for each chunk. Rust's `slice::chunks` is a zero-allocation lazy iterator that yields contiguous sub-slices — idiomatic and O(1) per chunk boundary.

2. **Byte arithmetic for char mapping:** Both languages use the same `'z' - (c - 'a')` formula. OCaml works with `Char.code` (int) and converts back with `Char.chr`. Rust casts `char` to `u8`, does byte arithmetic, and casts back to `char` — more explicit about the ASCII-only domain of the operation.

3. **Self-inverse ciphers share code:** Because `atbash(atbash(x)) = x`, both `encode` and `decode` call the same `atbash_char` function. `decode` only differs by filtering whitespace first (to ignore grouping spaces), then applying the same map. This is cleaner than duplicating the mapping logic.

4. **`filter_map` is universal:** OCaml's `Seq.filter_map` and Rust's `Iterator::filter_map` have identical semantics: apply a function that returns `Option`, keep only `Some` values, unwrap them. The OCaml `|>` pipeline and Rust's method chain are stylistically equivalent.

5. **`collect` vs `String.init` / `String.concat`:** OCaml builds each chunk string with `String.init (List.length chunk) (List.nth chunk)` — indexing into a list, which is O(n) per character. Rust iterates the `&[char]` chunk directly with `.iter().collect::<String>()`, which is O(n) total and allocates exactly once per chunk.

## When to Use Each Style

**Use idiomatic Rust (`chunks`) when:** you need to group data into fixed-size windows — it is the standard idiom, clearer to read, and the most performant approach.

**Use recursive Rust (`split_at`) when:** you are explicitly demonstrating the OCaml parallel, teaching the recursive decomposition pattern, or when the grouping logic has variable chunk sizes that make `chunks` unsuitable.
