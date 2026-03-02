# Example 002: Last Two Elements

**Difficulty:** ⭐ Beginner  
**Category:** Lists & Higher-Order Functions  
**OCaml Source:** 99 Problems #2  

## Problem

Find the last two elements of a list.

## Learning Outcomes

- Multi-element pattern matching
- Handling insufficient elements
- Slice pattern matching in Rust

## Key Differences

- OCaml: Recursive pattern match on cons cells
- Rust: Direct slice pattern `[.., a, b]`
- Both return `Option` for safety
