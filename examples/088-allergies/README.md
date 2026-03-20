[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 088 — Allergies
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Given a score (up to 255), determine which allergens a person reacts to by decoding a bitflag. Each of the eight allergens maps to a power-of-two score. Implement `is_allergic_to(allergen, score) -> bool` and `allergies(score) -> Vec<Allergen>` using bitwise AND. Compare with OCaml's `land` operator and list filtering.

## Learning Outcomes

- Use bitwise AND (`&`) to test individual bits in a compact integer representation
- Map each enum variant to a power-of-two score with a `match` or a shift expression
- Use `Allergen::ALL` constant array to iterate all variants for `allergies`
- Understand why a `u32` score (not `u8`) avoids overflow when scores combine
- Map Rust's bitflag pattern to OCaml's `land` (logical AND) integer operator
- Recognise the bitflag design pattern used in permissions, flags, and sets

## Rust Application

`Allergen::score(self) -> u32` returns the power-of-two associated with each variant. `is_allergic_to(allergen, score)` computes `score & allergen.score() != 0`. The `allergies(score)` function filters `Allergen::ALL` — a `const` array — by `is_allergic_to`. Using a `const` array rather than a `Vec` means the full allergen list is in static memory with no heap allocation. Scores above 255 are silently masked: only bits 0–7 are meaningful.

## OCaml Approach

OCaml uses `land` (bitwise AND): `score land allergen_score allergen <> 0`. `allergies score` is `List.filter (fun a -> is_allergic_to a score) all` where `all` is a manually defined list. The logic is identical; the differences are syntactic (`land` vs `&`, list vs array) and minor.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Bitwise AND | `score & allergen.score() != 0` | `score land allergen_score allergen <> 0` |
| All variants | `const ALL: [Allergen; 8]` | `let all = [Eggs; …]` (list) |
| Filter | `.iter().filter(…).collect()` | `List.filter (fun a -> …) all` |
| Score type | `u32` | `int` |
| Variant copy | `#[derive(Copy)]` | Value type by default |
| Score mapping | `match self { Eggs => 1, … }` | `function \| Eggs -> 1 \| …` |

The bitflag pattern is efficient and compact: eight boolean attributes stored in a single byte. The same technique underpins Unix file permissions, network flags, and capability bitmasks. The enum-to-power-of-two mapping with a `const` ALL array is a clean Rust idiom for this use case.

## Exercises

1. Replace the `score` match with a bit-shift: `1u32 << (self as u32)` for a zero-indexed enum. Verify the same scores result.
2. Add a `Allergen::from_score(score: u32) -> Option<Allergen>` that maps a single power-of-two back to a variant.
3. Implement `allergies_count(score: u32) -> usize` using `.count_ones()` on the score casted to `u8`.
4. Extend to 16 allergens by adding 8 more variants and changing the score type to `u16`.
5. In OCaml, implement a `has_all : allergen list -> int -> bool` predicate that returns `true` only when every allergen in the list is active in the score.
