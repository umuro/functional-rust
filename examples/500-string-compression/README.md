📖 **[View on hightechmind.io →](https://hightechmind.io/rust/500-string-compression)**

---

# String Compression — Run-Length Encoding
**Difficulty:** ⭐  
**Category:** Functional Programming  



Run-length encoding (RLE) compresses strings by replacing consecutive repeated characters with a `(count, char)` pair, implemented with `fold` for encoding and `fold` for decoding — both purely functional.

## Problem Statement

Run-length encoding is one of the simplest lossless compression schemes: instead of storing `"aaabbbcc"` (8 bytes), store `[(3,'a'), (3,'b'), (2,'c')]` or `"3a3b2c"` (6 chars). It is used in TIFF/BMP image formats (runs of same-color pixels), fax transmission (ITU-T T.4, runs of black/white pixels), PCX graphics, and DNA sequence compression. While LZ77/Huffman/DEFLATE are more powerful for general text, RLE is O(N) time/space and trivially invertible — ideal for data with long runs.

## Learning Outcomes

- Encode a string into `Vec<(usize, char)>` using `fold` with a `(current_char, count, acc)` accumulator
- Decode by folding `(count, char)` pairs into a `String` with repeated `push`
- Format encoded output as human-readable `"3a3b2c"` with `map` + `collect`
- Handle the edge case of an empty string (no fold initial value)
- Recognise the pattern of seeding `fold` with the first element to avoid an Option

## Rust Application

`encode` uses `fold` with a three-tuple accumulator to accumulate runs:

```rust
fn encode(s: &str) -> Vec<(usize, char)> {
    let mut chars = s.chars();
    let first = match chars.next() { None => return vec![], Some(c) => c };
    let (cur, count, mut acc) =
        chars.fold((first, 1usize, Vec::new()), |(cur, count, mut acc), c| {
            if c == cur { (cur, count + 1, acc) }
            else { acc.push((count, cur)); (c, 1, acc) }
        });
    acc.push((count, cur));
    acc
}
```

`decode` reconstructs the string by pushing each char `count` times:

```rust
fn decode(pairs: &[(usize, char)]) -> String {
    pairs.iter().fold(String::new(), |mut s, &(n, c)| {
        for _ in 0..n { s.push(c); }
        s
    })
}
```

## OCaml Approach

```ocaml
let encode s =
  match String.to_seq s |> List.of_seq with
  | [] -> []
  | first :: rest ->
    let (cur, count, acc) = List.fold_left (fun (cur, count, acc) c ->
      if c = cur then (cur, count + 1, acc)
      else (c, 1, (count, cur) :: acc)) (first, 1, []) rest in
    List.rev ((count, cur) :: acc)

let decode pairs =
  let buf = Buffer.create 64 in
  List.iter (fun (n, c) ->
    for _ = 1 to n do Buffer.add_char buf c done) pairs;
  Buffer.contents buf
```

OCaml's `List.fold_left` is equivalent to Rust's `Iterator::fold`; the accumulator pattern is identical.

## Key Differences

1. **`fold` accumulator**: Both Rust and OCaml use a three-element fold accumulator `(current, count, result_so_far)`; the seeding pattern (extract first element before fold) is the same.
2. **Decode strategy**: Rust's `fold` accumulates into a `String` via `push`; OCaml uses `Buffer` + `iter`. Both are O(N) with one allocation.
3. **`String::with_capacity`**: The decode result size is known (`pairs.iter().map(|(n,_)| n).sum()`); pre-allocating avoids reallocation. OCaml's `Buffer.create` hint is advisory.
4. **`Iterator::flat_map` decode**: An alternative Rust decode: `pairs.iter().flat_map(|(n,c)| std::iter::repeat(*c).take(*n)).collect()` — more functional but slightly less efficient.

## Exercises

1. **Encode to string**: Implement `fn encode_str(s: &str) -> String` that formats directly as `"3a3b2c"` without an intermediate `Vec`.
2. **Decode from string**: Implement `fn decode_str(s: &str) -> Option<String>` that parses `"3a3b2c"` by alternating between `parse::<usize>()` runs and char reads.
3. **Compression ratio**: Write a benchmark comparing RLE compression on (a) random ASCII text, (b) repeated patterns like `"abcabcabc"`, and (c) binary-like data — measure when RLE expands vs. compresses.
