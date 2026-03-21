📖 **[View on hightechmind.io →](https://hightechmind.io/rust/006-palindrome-check)**

---

# 006 — Palindrome Check
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

A palindrome reads the same forwards and backwards — "racecar", "level", "A man a plan a canal Panama". Palindrome detection is a classic string processing problem that exercises iterator bidirectionality: comparing a sequence against its reverse without necessarily materializing the reverse. It appears in bioinformatics (DNA palindromes are recognition sites for restriction enzymes), signal processing (palindromic sequences), and string interview problems.

The problem also illustrates an important distinction between eager (allocating the reversed string) and lazy (comparing iterators without allocation) approaches — a recurring theme in functional programming where correctness comes first and optimization second.

## Learning Outcomes

- Compare a sequence against its reverse using iterator bidirectionality
- Distinguish between allocating (`collect` to `String`) and non-allocating (`.eq(rev())`) approaches
- Handle Unicode correctly using `chars()` rather than byte-indexing
- Normalize input (lowercase, filter non-alphanumeric) for real-world palindrome checks
- Use Rust's `DoubleEndedIterator` for O(1) `rev()` without allocation

## Rust Application

`is_palindrome_rev` allocates a reversed `String` and compares with `==` — simple and clear. `is_palindrome_iter` uses `s.chars().eq(s.chars().rev())` — zero allocation, leveraging that `Chars` implements `DoubleEndedIterator`. `is_palindrome_clean` demonstrates production-quality normalization: filter to alphanumeric, lowercase each character with `to_lowercase().next().unwrap()`, collect to a `Vec<char>` to enable comparing against its own `rev()`. This correctly handles "A man, a plan, a canal: Panama".

## OCaml Approach

OCaml's `String` module lacks a built-in `String.rev`, but it is easily constructed: `let rev s = String.init (String.length s) (fun i -> s.[String.length s - 1 - i])`. Character-level access is `s.[i]`. For the clean version, OCaml uses `String.to_seq` and `Seq.filter`, then compares sequences. The functional style avoids mutation: build a clean char list, compare it with its reverse using `List.for_all2`.

## Key Differences

1. **Zero-copy rev**: Rust's `DoubleEndedIterator` allows `rev()` on any iterator without allocation. OCaml's `String.rev` allocates a new string; you need a custom approach for zero-copy comparison.
2. **Unicode**: Rust's `.chars()` correctly iterates Unicode scalar values. Naive byte indexing (`s.as_bytes()`) breaks on multibyte characters. OCaml's `s.[i]` is byte-level by default; use `Uchar` for Unicode.
3. **Normalization**: Both languages require explicit lowercasing and filtering. Rust's `char::to_lowercase()` returns an iterator (handling ligatures that expand to multiple chars). OCaml's `Char.lowercase_ascii` is simpler but ASCII-only.
4. **Comparison**: Rust's iterator `.eq()` short-circuits at the first difference. OCaml's `=` on lists is structural equality that traverses the full list.

## Exercises

1. **Largest palindrome substring**: Write `largest_palindrome(s: &str) -> &str` that returns the longest palindromic substring using Manacher's algorithm or naive O(n²) expansion.
2. **Palindrome pairs**: Given a list of words, find all pairs `(i, j)` where concatenating words `i` and `j` forms a palindrome.
3. **Streaming check**: Write `is_palindrome_stream(iter: impl Iterator<Item=char>) -> bool` that checks if a character stream is a palindrome by collecting to `Vec<char>` then comparing with its reverse.
