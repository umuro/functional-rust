📖 **[View on hightechmind.io →](https://hightechmind.io/rust/019-rotate-left)**

---

# 019 — Rotate a List to the Left
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Rotating a list left by n positions — moving the first n elements to the end — is a fundamental circular buffer operation. It appears in round-robin schedulers (rotating the job queue), circular queues, cryptographic operations (bit rotation in SHA, AES), polynomial arithmetic (cyclic shift), and visual animation (scrolling backgrounds).

The efficient algorithm avoids creating intermediate lists: for a list of length `len`, rotating by `n % len` combines two split-and-concatenate operations. This is equivalent to the "juggling algorithm" and the "reversal algorithm" for in-place rotation.

## Learning Outcomes

- Implement rotation as split + concatenate: `rotate(v, n) = v[n..] ++ v[..n]`
- Handle negative rotations and n > len by using modular arithmetic `n % len`
- Use `[n..].to_vec()` extended with `[..n]` for clear, correct rotation
- Understand the connection between rotation and circular buffers
- Apply the reversal algorithm as an alternative O(n) in-place approach

## Rust Application

The idiomatic approach: `let n = n % v.len(); let mut result = v[n..].to_vec(); result.extend_from_slice(&v[..n]); result`. The `n % v.len()` normalization handles rotations larger than the list. An alternative uses slice concatenation: `[&v[n..], &v[..n]].concat()`. For in-place rotation, `v.rotate_left(n)` is built into `Vec` and is O(n) with no allocation.

## OCaml Approach

OCaml's version: `let rotate lst n = let len = List.length lst in let n = ((n mod len) + len) mod len in let (left, right) = split lst n in right @ left`. The `((n mod len) + len) mod len` handles negative rotations correctly. `split` from example 017 decomposes the list, and `@` concatenates. OCaml's `@` is O(|left|), making this O(n) overall.

## Key Differences

1. **In-place rotation**: Rust's `v.rotate_left(n)` mutates `Vec` in place using a three-reversal algorithm — O(n) time, O(1) space. OCaml's immutable lists always allocate new structure.
2. **Negative rotation**: Rust's in-place `rotate_left` panics on n > len; normalize first. OCaml needs `((n mod len) + len) mod len` to handle negative values.
3. **`@` vs `extend_from_slice`**: OCaml's `right @ left` is O(|right|) because it copies the right list. Rust's `extend_from_slice` appends in O(n) but avoids an intermediate allocation if capacity allows.
4. **`split_at` performance**: Rust's split is O(1) (pointer arithmetic). OCaml's `List.length` + `split` are O(n) each.

## Exercises

1. **Rotate right**: Write `rotate_right(v: &[i32], n: usize) -> Vec<i32>` using `rotate_left` with appropriate adjustment.
2. **Caesar cipher via rotation**: A Caesar cipher on the alphabet is a rotation of 26 characters. Write `caesar_encrypt(text: &str, shift: usize) -> String` using `rotate_left` on the alphabet.
3. **Detect rotation**: Write `is_rotation(a: &[i32], b: &[i32]) -> bool` that returns true if `b` is a rotation of `a`. Use the classic trick of checking if `b` is a subarray of `a ++ a`.
