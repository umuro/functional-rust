📖 **[View on hightechmind.io →](https://hightechmind.io/rust/063-run-length-encoding)**

---

# 063 — Run-Length Encoding

## Problem Statement

Run-length encoding (RLE) compresses consecutive repeated characters: "AABCCCDEEEE" → "2AB3CD4E". It is one of the oldest and simplest compression algorithms, used in fax machines (CCITT T.4 standard, 1980), BMP image files, PCX format, and as the basis for more sophisticated codecs. The decode-recode round-trip test is a classic example of property-based testing.

This version operates on strings rather than generic lists, demonstrating character-level iteration in Rust: collecting to `Vec<char>` for indexed access, building output with `String::push_str`, and decoding by parsing runs of digits followed by a character.

## Learning Outcomes

- Iterate over a string as `Vec<char>` for indexed access
- Detect run boundaries and emit count+char pairs
- Decode by parsing alternating digit sequences and characters
- Handle the edge case: count=1 (single character, no prefix)
- Implement the round-trip invariant: `decode(encode(s)) == s`

- Implement both two-pass (pack then count) and single-pass (fold with run tracking) RLE encoding
- Use `encode_fold` with `acc.last_mut()` for single-pass encoding that avoids intermediate allocation

## Rust Application

`encode` collects chars to `Vec<char>`, iterates with an index tracking the current run, and builds the result string. When a run ends (different char or end of string), pushes the count (if > 1) and the character. `decode` iterates chars, accumulating a `count` from digit characters and emitting `repeat` characters when a letter is encountered. The `for _ in 0..repeat { result.push(c); }` loop handles the expansion.

## OCaml Approach

OCaml's string version: `let encode s = let n = String.length s in let buf = Buffer.create n in (* ... iterate with a counter ... *) Buffer.contents buf`. OCaml's `String.get` accesses characters by index. For decoding: `String.to_seq s |> Seq.fold_left (fun (count, result) c -> ...)`. The `Buffer` type accumulates the output string efficiently.

## Key Differences

1. **`chars()` vs `String.get`**: Rust's `s.chars()` returns a `Chars` iterator over Unicode scalar values. `s.as_bytes()[i]` gives bytes. OCaml's `s.[i]` or `String.get s i` gives bytes. Both should use character-level access for Unicode safety.
2. **String building**: Rust uses `String::push` and `String::push_str`. OCaml's `Buffer` is equivalent — append-efficient string accumulation. Both avoid O(n²) repeated concatenation.
3. **Digit parsing**: Rust uses `c.is_ascii_digit()` and arithmetic `c as u32 - '0' as u32`. OCaml: `Char.code c - Char.code '0'`. Same approach.
4. **`count * 10 + digit`**: Multi-digit run lengths (10+) require accumulating digits: `count = count * 10 + digit`. Both implementations handle this the same way.

1. **`dedup` vs RLE:** `dedup()` removes all consecutive duplicates. RLE counts them. Different operations for different use cases — compression uses RLE; uniqueness uses dedup.
2. **`Itertools::chunk_by`:** The `itertools` crate provides `chunk_by(|a, b| a == b)` which groups consecutive equal elements. This is the iterator-based `pack` operation from example 009.
3. **Generic vs concrete:** Making `encode<T: PartialEq + Clone>` generic handles any element type, not just characters. OCaml's `'a rle` is also polymorphic.
4. **Performance:** Single-pass encoding with `fold` is faster than two-pass (group then count) because it avoids intermediate allocation. For string RLE specifically, byte-level iteration (`bytes()`) is faster than `chars()` for ASCII input.

## Exercises

1. **Generic RLE**: Adapt to work on `&[T]` instead of `&str`, returning a `String`-like encoding. What type should the output be for generic T?
2. **Streaming codec**: Write an `RleEncoder` struct with a `push(c: char) -> Option<String>` method that emits encoded chunks when runs complete, enabling streaming use.
3. **Benchmark**: Compare encoding performance on "AAAAABBBCCCCC..." (long runs) vs "ABCDEFGH..." (no runs). When is RLE beneficial vs harmful?

4. **String RLE**: Implement `rle_encode_str(s: &str) -> String` that encodes a string in a compact format like `"aaa bb c"` → `"3a 2b c"` and `rle_decode_str` that reverses it.
5. **Image RLE**: Implement `encode_image(pixels: &[u8]) -> Vec<(u8, u8)>` for a grayscale image encoded as a flat byte array — this is the actual BMP/PCX compression format.
