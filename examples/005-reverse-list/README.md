# Example 005: Reverse a List

**Difficulty:** ⭐ Beginner  
**Category:** List Transformation  
**OCaml Source:** 99 Problems #5  

## Problem

Reverse a list.

## Learning Outcomes

- Accumulator pattern for reversing
- In-place vs functional reversal
- Iterator adapters

## Key Differences

- OCaml: Tail-recursive accumulator, creates new list
- Rust: `.rev()` iterator (lazy), or `.reverse()` (in-place mutation)
- Rust offers both functional and imperative approaches
