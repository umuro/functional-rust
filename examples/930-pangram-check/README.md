📖 **[View on hightechmind.io →](https://hightechmind.io/rust/930-pangram-check)**

---

# 930-pangram-check — Pangram Check
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A pangram is a sentence containing every letter of the alphabet at least once. "The quick brown fox jumps over the lazy dog" is the famous example, used for font demonstrations and keyboard testing. Checking for a pangram is a set-membership problem: does the set of distinct lowercase letters in the sentence contain all 26 letters? Three approaches illuminate different algorithmic ideas: set-based (clear and general), bitflag (memory-minimal, no heap allocation), and recursive (OCaml-style demonstration). The bitflag approach — one bit per letter in a 32-bit integer — is a classic compact set representation for small fixed-universe sets.

## Learning Outcomes

- Use `HashSet<char>` to collect unique letters and check for 26 distinct entries
- Implement the bitflag approach using `u32` as a 26-bit set — zero heap allocation
- Recognize `1 << idx` as the set-insert operation and `(1 << 26) - 1` as the "all set" check
- Implement a recursive approach mirroring OCaml's style
- Compare the three approaches' trade-offs in readability, allocation, and performance

## Rust Application

`is_pangram` collects lowercase alphabetic characters into a `HashSet<char>` and checks `.len() == 26`. `is_pangram_bitflag` uses `seen |= 1 << (c as u32 - 'a' as u32)` for each letter and checks `seen == (1 << 26) - 1` — no allocation. `is_pangram_recursive` uses `has_all` to check each letter from 'a' to 'z' recursively. The bitflag version is fastest (no allocation, single pass, O(1) membership), `HashSet` is clearest, recursive is most OCaml-like.

## OCaml Approach

OCaml `is_pangram s = let letters = s |> String.lowercase_ascii |> String.to_seq |> Seq.filter (fun c -> c >= 'a' && c <= 'z') |> List.of_seq |> List.sort_uniq compare in List.length letters = 26`. Or using a `Hashtbl`. The bitflag approach: `let check_pangram s = let seen = ref 0 in String.iter (fun c -> if c >= 'a' && c <= 'z' then seen := !seen lor (1 lsl (Char.code c - Char.code 'a'))) s; !seen = (1 lsl 26) - 1`. OCaml's `lsl` for left shift vs Rust's `<<`.

## Key Differences

1. **Set implementation**: Rust `HashSet<char>` vs OCaml `Hashtbl` or `List.sort_uniq` — same semantics, different APIs.
2. **Bitflag safety**: Rust's `u32` bitfield uses safe arithmetic; OCaml's `lsl` on native integers can overflow (63-bit on 64-bit systems).
3. **Recursive style**: Rust's recursion over `u8` (letter codes) mirrors OCaml's style but requires explicit bounds checking.
4. **Char comparison**: Rust uses `'a' as u32` numeric conversion; OCaml uses `Char.code 'a'`.

## Exercises

1. Generalize `is_pangram` to `covers_all<T: Hash + Eq>(text: impl IntoIterator<Item=T>, required: impl IntoIterator<Item=T>) -> bool`.
2. Implement `pangram_missing_letters(s: &str) -> Vec<char>` that returns the alphabetically sorted list of missing letters.
3. Write `shortest_pangram(words: &[&str]) -> Option<Vec<&str>>` that finds the minimum-word-count subset that forms a pangram.
