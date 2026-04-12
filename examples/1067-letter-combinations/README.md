📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1067-letter-combinations)**

---

# 1067-letter-combinations — Phone Keypad Letter Combinations
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Given a phone number keypad string (digits 2–9), generate all possible letter combinations. This is a classic Cartesian product problem: each digit maps to a set of letters, and you want all combinations taking one letter from each digit's set.

The problem appears in T9 predictive text input (legacy phones), QR code testing tools that enumerate all encodings, and any system generating all strings from a grammar.

## Learning Outcomes

- Implement Cartesian product via backtracking
- Implement the same via iterative queue-based expansion
- Understand the connection between Cartesian products and nested loops
- Handle edge cases (empty input, single digit)
- Connect to formal language theory: this generates words in a regular language

## Rust Application

`src/lib.rs` uses a `PHONE_MAP: &[&str]` lookup table indexed by digit value. `letter_combos` uses backtracking: for each digit, try each letter in its mapping and recurse. `letter_combos_iter` uses an iterative approach: start with `[""]`, and for each digit, expand each existing combination by appending each letter.

The iterative approach is the explicit Cartesian product: for each layer, multiply the current set by the new letter set. This is the pattern behind SQL `CROSS JOIN`.

## OCaml Approach

```ocaml
let phone_map = [|""; ""; "abc"; "def"; "ghi"; "jkl"; "mno"; "pqrs"; "tuv"; "wxyz"|]

let letter_combos digits =
  if digits = "" then []
  else
    String.fold_left (fun acc d ->
      let letters = phone_map.(Char.code d - Char.code '0') in
      if acc = [] then List.map (String.make 1) (String.to_seq letters |> List.of_seq)
      else List.concat_map (fun combo ->
        List.map (fun c -> combo ^ String.make 1 c)
          (String.to_seq letters |> List.of_seq)
      ) acc
    ) [] (String.to_seq digits |> List.of_seq)
```

OCaml's `List.concat_map` provides the Cartesian product expansion cleanly.

## Key Differences

1. **String mutation**: Rust uses `String::push` and `String::pop` for in-place character manipulation during backtracking; OCaml uses string concatenation.
2. **Iterative expansion**: Both iterative versions use a collection that grows by Cartesian product; Rust uses a `Vec<String>` and extends it explicitly.
3. **`String::pop`**: Rust's `current.pop()` undoes the last `push(c)` for backtracking; OCaml's immutable string approach naturally supports backtracking.
4. **`PHONE_MAP` indexing**: Both use digit-minus-base offset (`digit - '0'`) to index the map — identical arithmetic.

## Exercises

1. Extend the keypad map to include `*` and `#` symbols and test with phone numbers including these characters.
2. Generate combinations filtered by a dictionary: return only letter combinations that form valid English words.
3. Implement `letter_combos_lazy(digits: &str) -> impl Iterator<Item=String>` that generates combinations lazily without collecting all into a `Vec`.
