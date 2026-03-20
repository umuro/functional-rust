📖 **[View on hightechmind.io →](https://hightechmind.io/rust/546-lifetime-reborrow)**

---

# Reborrowing Patterns
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Reborrowing is the implicit mechanism by which Rust creates a shorter-lived reference from a longer-lived one. When you pass `&mut x` to a function that takes `&mut i32`, Rust does not move the mutable reference — it creates a reborrow that lasts only for the function call. When the call returns, the original `&mut x` is available again. Without reborrowing, using `&mut` references would require move semantics — you could only use a mutable reference once. Understanding reborrowing explains why `&mut` chains work intuitively in practice.

## Learning Outcomes

- What reborrowing is: creating a shorter-lived reference from a longer-lived one
- How `&mut T` implicitly reborrows as `&T` when passed to a function expecting `&T`
- How explicit reborrow `&*r` creates a shared reference from a mutable one
- How method chains on `&mut self` work through repeated reborrowing
- Why reborrowing enables `&mut` references to be used multiple times in sequence

## Rust Application

`implicit_reborrow_demo` shows `let r = &mut x; read_value(r)` — `r` is reborrowed as `&i32` for the call, then becomes available again for `*r += 1`. `explicit_reborrow(r: &mut i32)` shows `let shared: &i32 = &*r` — manually creating a shared reborrow. `Counter::increment(&mut self)` demonstrates method chaining — each method call reborrows `self` for its duration. The source includes patterns for passing `&mut` to multiple functions sequentially.

Key patterns:
- `read_value(r)` — `r: &mut i32` reborrows as `&i32` for the call duration
- `&*r` — explicit reborrow from `&mut T` to `&T`
- Method chaining: each `self.method()` reborrows `self` for that call

## OCaml Approach

OCaml's `ref` values are always accessible and never "consumed" — there is no reborrow concept because mutation and reading are always available through the same reference:

```ocaml
let x = ref 42
let _ = !x            (* read *)
let () = incr x       (* mutate *)
let _ = !x            (* read again — no reborrow needed *)
```

## Key Differences

1. **Implicit mechanism**: Rust reborrowing happens automatically when needed — most users never think about it explicitly; OCaml has no equivalent because mutation and reading are always available.
2. **Reference reuse**: Rust `&mut` references can be used multiple times via reborrowing; without reborrowing, they would need to be moved on every use (like `Box`).
3. **Lifetime of reborrow**: The reborrow's lifetime is strictly shorter than the original — Rust enforces this statically; OCaml references have no lifetime hierarchy.
4. **Method chains**: Rust method chains on `&mut self` work through a sequence of reborrows at each method call boundary; OCaml method calls on mutable values work without restriction.

## Exercises

1. **Sequential reborrows**: Write a function that takes `r: &mut Vec<i32>`, calls `r.len()` (shared reborrow), then `r.push(0)` (mutable reborrow), then `r.len()` again — add comments showing the reborrow sequence.
2. **Explicit reborrow function**: Implement `fn peek_then_pop(v: &mut Vec<i32>) -> Option<(i32, i32)>` that reads `v.last()` (shared reborrow), then calls `v.pop()` (mutable), returning both values.
3. **Nested method chain**: Create a `Builder` struct with `&mut self` methods that mutate fields and return `&mut Self` — demonstrate chaining four method calls in one expression.
