📖 **[View on hightechmind.io →](https://hightechmind.io/rust/929-palindrome-check)**

---

# 929-palindrome-check — Palindrome Check
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A palindrome reads the same forwards and backwards: "racecar", [1,2,1], "madam". Palindrome checking exercises iterator-based bidirectional comparison, a fundamental algorithm pattern. The naive approach reverses the sequence and compares — O(n) time and O(n) space. The efficient approach compares from both ends using a double-ended iterator — O(n) time and O(1) space. This algorithm appears in DNA sequence analysis (palindromic restriction enzyme sites), string processing, and as a classic interview problem. Rust's `.eq(iter.rev())` idiom makes it a clean one-liner.

## Learning Outcomes

- Use `.iter().eq(slice.iter().rev())` for a clean one-liner palindrome check
- Understand that this compares two iterators element-by-element with O(n) passes
- Implement the manual comparison approach for educational purposes
- Apply the generic palindrome check to both `&[i32]` and `&str` character slices
- Compare with OCaml's `list = List.rev list` approach

## Rust Application

`is_palindrome<T: PartialEq>(list: &[T])` uses `list.iter().eq(list.iter().rev())` — the two iterators traverse from opposite ends and `.eq()` short-circuits on first mismatch. `is_palindrome_manual` reverses into a new `Vec` then compares slices. Both are O(n); the first is slightly more elegant. The generic `T: PartialEq` bound works for integers, characters, strings, or any equatable type. For strings: `s.chars().eq(s.chars().rev())` handles Unicode correctly.

## OCaml Approach

OCaml: `let is_palindrome xs = xs = List.rev xs` — reverse and compare using structural equality. This creates a new reversed list (O(n) allocation) then compares. More efficient: pattern matching from both ends is awkward with singly-linked lists — it requires converting to an array. `let is_palindrome_arr xs = let arr = Array.of_list xs in let n = Array.length arr in let rec go i j = i >= j || arr.(i) = arr.(j) && go (i+1) (j-1) in go 0 (n-1)`.

## Key Differences

1. **One-liner**: Rust `list.iter().eq(list.iter().rev())` is elegant and generic; OCaml `xs = List.rev xs` is equally elegant but allocates a reversed copy.
2. **Bidirectional**: Rust's approach conceptually traverses from both ends via two iterators; OCaml reverses first then compares linearly.
3. **Generic**: Rust's `<T: PartialEq>` works for any type; OCaml's structural `=` also works for any type but may be slower for complex types.
4. **Short-circuit**: Rust's `.eq()` short-circuits on first mismatch; OCaml's list equality also short-circuits.

## Exercises

1. Implement `is_palindrome_str(s: &str) -> bool` that handles Unicode correctly (comparing grapheme clusters, not bytes).
2. Write `longest_palindromic_substring(s: &str) -> &str` that finds the longest palindromic substring.
3. Implement `make_palindrome<T: Clone>(prefix: &[T]) -> Vec<T>` that appends the reverse of the prefix to make a palindrome.
