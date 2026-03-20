📖 **[View on hightechmind.io →](https://hightechmind.io/rust/012-decode-rle)**

---

# 012 — Decode Run-Length Encoding
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Decoding run-length encoding (OCaml 99 Problems #12) is the inverse of encoding: given a sequence of `(count, element)` or `element` items, reconstruct the original uncompressed sequence. This is the decompression side of a codec — used in fax machines (CCITT T.4), BMP images, PCX format, and simple network protocols.

Decoding exercises `flat_map`: each encoded item expands into zero or more output elements. `flat_map` (also called `concat_map` in OCaml) is the fundamental operation for structure-expanding transformations. It generalizes `map` by allowing each input to produce multiple outputs, and it is the monadic bind operation for lists.

## Learning Outcomes

- Use `flat_map` to expand encoded items into multiple output elements
- Use `std::iter::repeat(x).take(n)` to generate `n` copies of a value
- Understand `flat_map` as monadic bind for `Vec`/iterators
- Compare eager (`flat_map + collect`) vs recursive decoding approaches
- Recognize the encode/decode round-trip invariant for testing

## Rust Application

`decode` uses `flat_map` over the encoded slice: for each `One(x)` it produces `vec![x.clone()]`, for each `Many(n, x)` it produces `vec![x.clone(); n]`. The iterator-based `decode_iter` uses `std::iter::repeat(value).take(count)` which is more idiomatic and avoids an intermediate `Vec` per item. The `flat_map` approach makes the logic declarative: "for each encoded item, expand it to its elements and concatenate all results".

## OCaml Approach

OCaml's decode uses `List.concat_map` (or `List.flatten (List.map f lst)`). The typical implementation is: `let decode lst = List.concat_map (function One x -> [x] | Many (n, x) -> List.init n (fun _ -> x)) lst`. `List.init n f` creates a list of length `n` by calling `f` with indices 0 to n-1. The recursive approach matches on the head and expands it before processing the tail.

## Key Differences

1. **`flat_map` naming**: Rust calls it `flat_map`, OCaml calls it `concat_map` (stdlib 4.10+) or implements it as `List.flatten (List.map f lst)`. Both mean the same operation: map then flatten one level.
2. **`repeat` vs `List.init`**: Rust's `std::iter::repeat(x).take(n)` is more direct than OCaml's `List.init n (fun _ -> x)`. Both produce n copies without mutation.
3. **Clone requirement**: Rust needs `T: Clone` to produce multiple copies from a reference. OCaml's GC allows sharing the same value across all copies without cloning.
4. **Laziness**: Rust's `flat_map(...).collect()` is lazy until `collect`. OCaml's `List.concat_map` is eager — it builds the full list immediately.

## Exercises

1. **Round-trip test**: Write a property-based test that generates random `Vec<char>` inputs, encodes them with `encode_modified` from example 011, decodes with `decode`, and asserts equality.
2. **Streaming decode**: Rewrite decode to accept `impl Iterator<Item=RleItem<T>>` and return `impl Iterator<Item=T>` without collecting to `Vec` at any step. Use `flat_map` on the input iterator.
3. **RLE of RLE**: What happens when you RLE-encode an already-encoded RLE sequence? Write a function that detects whether further compression is beneficial.
