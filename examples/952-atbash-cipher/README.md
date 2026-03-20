**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

[atbash-cipher on hightechmind.io](https://hightechmind.io/posts/functional-rust/atbash-cipher)

---

## Problem Statement

Implement the Atbash cipher: map each letter to its reverse in the alphabet (a→z, b→y, ..., z→a), pass digits through unchanged, and discard all other characters. For encoding, group the result into five-character chunks separated by spaces. Decoding strips spaces and applies the same bijective mapping. Implement both an iterator pipeline version and a recursive grouping variant.

## Learning Outcomes

- Implement `atbash_char(c) -> Option<char>` as a pure character mapping using `b'z' - (c as u8 - b'a')`
- Chain `.to_lowercase().chars().filter_map(atbash_char)` for encoding
- Use `.chunks(5)` on a collected `Vec<char>` to group into five-character blocks
- Implement decoding as the same pipeline (Atbash is its own inverse — an involution)
- Implement a recursive grouping function as an alternative to `.chunks(5)`

## Rust Application

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

pub fn decode(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter_map(atbash_char)
        .collect()
}
```

The encoding formula `b'z' - (c as u8 - b'a')` computes the mirror position: 'a' (0) maps to 'z' (25), 'b' (1) maps to 'y' (24), etc. Since `'a' + 'z' = 25 + 0 = 25` and `'b' + 'y' = 24 + 1 = 25`, the mapping satisfies `position(encode(c)) + position(c) = 25`, making it an involution.

`.chunks(5)` on a `Vec<char>` produces subslices. `.join(" ")` on the resulting `Vec<String>` inserts spaces between groups. The decode pipeline simply filters whitespace before applying the same character map.

## OCaml Approach

```ocaml
let atbash_char c =
  if c >= 'a' && c <= 'z' then
    Some (Char.chr (Char.code 'z' - (Char.code c - Char.code 'a')))
  else if c >= '0' && c <= '9' then Some c
  else None

let encode input =
  let chars =
    String.to_seq (String.lowercase_ascii input)
    |> Seq.filter_map atbash_char
    |> List.of_seq
  in
  let rec group = function
    | [] -> []
    | cs ->
      let (chunk, rest) = List.filteri (fun i _ -> i < 5) cs,
                          List.filteri (fun i _ -> i >= 5) cs in
      String.concat "" (List.map (String.make 1) chunk) :: group rest
  in
  String.concat " " (group chars)

let decode input =
  String.to_seq input
  |> Seq.filter (fun c -> c <> ' ')
  |> Seq.filter_map atbash_char
  |> String.of_seq
```

OCaml's `Seq.filter_map` is the lazy equivalent of Rust's `.filter_map()`. `String.of_seq` collects a `char Seq.t` into a `string` directly — no intermediate `Vec<char>`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Character arithmetic | `b'z' - (c as u8 - b'a')` | `Char.code 'z' - (Char.code c - Char.code 'a')` |
| Chunking | `.chunks(5)` on `Vec<char>` | Recursive split or `List.filteri` |
| String from chars | `.collect::<String>()` | `String.of_seq` or `String.concat "" (List.map ...)` |
| Join with separator | `.join(" ")` on `Vec<String>` | `String.concat " "` |
| Involution proof | The same `atbash_char` decodes | Identical |

Atbash is a substitution cipher where encode and decode are identical functions. The `.filter_map` idiom cleanly expresses "apply transformation, drop invalid characters" in a single pass.

## Exercises

1. Add support for non-ASCII Unicode letters by extending `atbash_char` to handle full Unicode alphabetic ranges.
2. Implement `encode_no_spaces` that skips the grouping step.
3. Verify the involution property: `decode(encode(s)) == normalize(s)` for all alphanumeric strings.
4. Implement ROT-13 using the same pattern — `b'a' + (c as u8 - b'a' + 13) % 26`.
5. Generalize to a `SubstitutionCipher` struct that takes an arbitrary mapping `[char; 26]` and implements both encode and decode.
