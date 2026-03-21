[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 086 — Space Age
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Given an age in seconds, calculate how old a person would be on each planet in the solar system, based on each planet's orbital period relative to Earth's year (31,557,600 seconds). Implement `age_on(planet: Planet, seconds: f64) -> f64` using both a `match`-based orbital period table and a lookup-array alternative.

## Learning Outcomes

- Use `Copy` enums to represent fixed domain sets (`Planet`) with no heap allocation
- Define a `const` array `Planet::ALL` for iterating over all variants
- Use `match` to return a compile-time float constant per variant
- Apply the formula: `age = seconds / (earth_year * orbital_period)` cleanly
- Compare Rust's `match`-based dispatch with OCaml's equivalent `function` match
- Understand when a data-driven lookup table is clearer than match arms

## Rust Application

`Planet` derives `Copy` — all eight variants carry no data. `orbital_period(self) -> f64` returns the planet's year length in Earth years via a match. `EARTH_YEAR_SECONDS` is a `const f64`. `age_on` divides `seconds` by the product of these two values. `Planet::ALL` is a `const` array of all eight variants, enabling `for planet in Planet::ALL { … }` for bulk computation without external iteration dependencies. The match approach makes the relationship between variant and data explicit at the source level.

## OCaml Approach

OCaml's version is nearly identical: a `type planet` variant, an `orbital_period` function using `function`, and `age_on` computing the quotient. The OCaml float operators are suffixed (`/.`, `*.`), making the arithmetic explicit. There is no `Copy` concept — variants are value types by default. OCaml lacks a `const` array of all constructors, so exhaustive iteration would require a manually defined list.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Variant copy | `#[derive(Copy)]` | Value type by default |
| Float ops | `*`, `/` (same as integer) | `*.`, `/.` (distinct operators) |
| Constant | `const EARTH_YEAR_SECONDS: f64` | `let earth_year_seconds = …` |
| All variants | `const ALL: [Planet; 8]` | Manual list or macro |
| Dispatch | `match self { … }` | `function \| Mercury -> …` |
| Code size | ~40 lines | ~15 lines |

The `Planet::ALL` constant is a useful pattern for any enum where you need to iterate over all variants. Without a derive macro like `strum`, Rust requires defining it manually — but as a `const` array it is zero-overhead and compile-time verified.

## Exercises

1. Add a `Display` implementation for `Planet` that returns the planet name as a string.
2. Implement `age_on_all(seconds: f64) -> [(Planet, f64); 8]` that returns the age on every planet.
3. Find which planet's year is closest to a given number of Earth years using `Planet::ALL.iter().min_by`.
4. Add a `Pluto` variant (orbital period 247.92065 Earth years) even though it is not officially a planet, and verify the formula still works.
5. In OCaml, define `all_planets : planet list` and write `age_on_all seconds` using `List.map`.
