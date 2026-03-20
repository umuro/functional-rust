📖 **[View on hightechmind.io →](https://hightechmind.io/rust/072-result-railway)**

---

# 072 — Railway-Oriented Programming
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Railway-oriented programming (ROP), coined by Scott Wlaschin, is a visual metaphor for monadic error handling. Picture a two-track railway: the "happy track" (Ok values) and the "error track" (Err values). Each function is a switch: if you are on the happy track and the function succeeds, you stay on it; if it fails, you switch to the error track and stay there until the end.

This pattern, implemented via `and_then` chaining, produces code that reads linearly without nested error handling. It is the functional equivalent of exception-based error flow but with explicit types. Used in F#, Rust, Scala, and any language where Result/Either is the error handling mechanism.

## Learning Outcomes

- Chain validation functions with `and_then` to form a railway
- Understand that each `and_then` step runs only on the happy track
- Use `validate_item`, `validate_quantity`, `validate_price` as railway switches
- Implement railway-style pipelines using both explicit `and_then` and `?`
- Recognize that the railway metaphor explains `Result` monad behavior

## Rust Application

Each validation function `fn validate_x(order: Order) -> Result<Order, String>` consumes the order and returns it unchanged (or an error). `validate_order_chain` uses `.and_then(validate_quantity).and_then(validate_price)` — each function is a switch on the railway. `validate_order_question` uses `?` to achieve the same effect. Both are identical in behavior; `?` is more readable for sequential pipelines.

## OCaml Approach

OCaml's version: `let validate_order order = let ( >>= ) = Result.bind in validate_item order >>= validate_quantity >>= validate_price`. The `>>=` operator makes the railway tracks explicit. With `let*`: `let* o = validate_item order in let* o = validate_quantity o in validate_price o`. Each step receives the validated order from the previous step.

## Key Differences

1. **Visual metaphor**: The railway metaphor makes `and_then` chains intuitive — you are either on the happy track or the error track. `?` makes the track switch implicit.
2. **Pass-through pattern**: Each validator takes and returns the full order (pass-through). This allows validators to be composed in any order without changing signatures.
3. **Error accumulation vs railway**: The railway pattern stops at the first error. For all-errors-at-once, use the `Validation` pattern (example 072-error-accumulation). Knowing when to use which is key.
4. **`>>=` in OCaml**: Defining `let (>>=) = Result.bind` locally in OCaml makes railway chains very readable: `a >>= f >>= g >>= h`. Rust's method syntax `a.and_then(f).and_then(g)` is equivalent.

## Exercises

1. **Enriched errors**: Change the error type to `struct ValidationError { field: String, message: String }`. Update the validators to include the field name. This enables structured error reporting.
2. **Parallel validation**: Combine the railway approach (sequential) with applicative validation (parallel): validate all fields simultaneously and accumulate errors, then construct the Order only if all pass.
3. **Undo railway**: Write a `rollback` function for a sequence of operations: if step 3 fails, undo steps 1 and 2. Model this with a stack of undo actions.
