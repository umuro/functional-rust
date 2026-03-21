📖 **[View on hightechmind.io →](https://hightechmind.io/rust/539-lifetime-nll)**

---

# Non-Lexical Lifetimes (NLL)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Before Rust 2018's Non-Lexical Lifetimes (NLL), the borrow checker used lexical scopes to determine when borrows ended — a borrow lasted until the end of the enclosing block, even if the borrowed value was never used after its last access point. This caused many correct programs to be rejected: you could not borrow from a `Vec`, compute something, then push to the `Vec` in the same block, even though the first borrow logically ended before the push. NLL (stabilized in Rust 2018) makes borrows end at their last use, not at the end of their enclosing scope.

## Learning Outcomes

- What NLL changed: borrows end at last use, not end of block
- How `let first = v[0]; v.push(6);` now compiles with NLL (borrow ends after `v[0]`)
- How NLL enables conditional borrows and mutation in the same function
- How `nll_match` demonstrates split borrows with pattern matching
- Where NLL matters most: iterative algorithms, in-place mutations, parser loops

## Rust Application

`nll_basic()` reads `v[0]` (creating a temporary borrow) then calls `v.push(6)` — before NLL, both would need to be in separate scopes; with NLL, the compiler sees the borrow ends after the first line. `nll_conditional` borrows `data.first()` only to copy the value; after the copy, the borrow ends and `data.push(42)` is legal. `nll_match` demonstrates match arms: the `Some(s)` arm borrows from `opt` and returns `s`, but NLL allows the borrow checker to understand this correctly.

Key patterns:
- Borrow ends at last use, not at `}`
- Copying a value from a borrow terminates the borrow immediately
- Conditional paths: borrow in one branch, mutation in another

## OCaml Approach

OCaml has no borrow checker — all mutation is safe through GC-managed references. The equivalent patterns work without restriction:

```ocaml
let nll_basic () =
  let v = ref [1; 2; 3; 4; 5] in
  let first = List.hd !v in
  v := !v @ [6];
  (first, !v)
```

There is no concept of a borrow ending — the GC handles everything.

## Key Differences

1. **Scope vs use**: Pre-NLL Rust used lexical scope boundaries; post-NLL Rust uses last-use points; OCaml has no borrow scope concept at all.
2. **Conditional borrows**: NLL makes Rust's borrow checker understand that borrows in one branch don't affect other branches; OCaml allows unrestricted mutation in all branches.
3. **Ergonomics impact**: NLL significantly reduced false rejections from the Rust borrow checker, making common patterns like read-then-mutate work without workarounds.
4. **Polonius**: NLL is followed by Polonius (an even more precise borrow checker) that will handle additional cases NLL cannot, such as borrowing in one loop iteration and releasing in another.

## Exercises

1. **Pre-NLL workaround**: Write the same logic as `nll_basic` using the pre-NLL workaround (explicit scope with `{}`) and verify both versions compile correctly.
2. **Loop borrow**: Write a loop that reads the minimum element of a `Vec<i32>`, removes it (using `retain`), and appends its square — demonstrate NLL allows this without explicit scoping.
3. **Match arms with NLL**: Implement `fn first_or_default<'a>(v: &'a Vec<String>, default: &'a str) -> &'a str` that returns the first element or default, using a match expression on `v.first()`.
