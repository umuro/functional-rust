📖 **[View on hightechmind.io →](https://hightechmind.io/rust/862-monad-transformers)**

---

# Monad Transformers

## Problem Statement

Real programs need multiple effects simultaneously: a computation might be fallible (Option/Result), AND need to read configuration (Reader), AND accumulate a log (Writer). Composing two monads directly doesn't work — `Option<Reader<Config, A>>` and `Reader<Config, Option<A>>` require different bind implementations. Monad transformers solve this by stacking effects: `OptionT<E, A> = Result<Option<A>, E>` adds optionality to Result. This enables writing code that handles both "not found" (None) and "failed" (Err) without explicit nesting. Used in: web frameworks (request handler: Result for errors, Option for optional fields, Reader for context), compiler pipelines, and effectful DSLs.

## Learning Outcomes

- Understand `OptionT<A, E> = Result<Option<A>, E>` as Option inside Result
- Implement `lift_option` and `lift_result` to promote values into the transformer
- Implement bind that threads both effects: `Err` short-circuits, `Ok(None)` propagates absence
- Recognize the tradeoff: transformers are complex; often explicit nesting is clearer
- Apply to: handlers returning `Result<Option<User>, DbError>` for "not found" vs. "db failed"

## Rust Application

```rust
// OptionT<A, E> = Result<Option<A>, E>
type OptionT<A, E> = Result<Option<A>, E>;

pub fn pure_opt<A, E>(a: A) -> OptionT<A, E> { Ok(Some(a)) }
pub fn none_opt<A, E>() -> OptionT<A, E> { Ok(None) }
pub fn lift_result<A, E>(r: Result<A, E>) -> OptionT<A, E> { r.map(Some) }
pub fn lift_option<A, E>(o: Option<A>) -> OptionT<A, E> { Ok(o) }
pub fn bind_opt<A, B, E>(
    ma: OptionT<A, E>,
    f: impl FnOnce(A) -> OptionT<B, E>,
) -> OptionT<B, E> {
    match ma {
        Err(e) => Err(e),
        Ok(None) => Ok(None),
        Ok(Some(a)) => f(a),
    }
}
```

The `bind_opt` handles three cases: `Err` propagates the error (Result effect); `Ok(None)` propagates absence (Option effect); `Ok(Some(a))` applies `f`. Both effects are preserved independently. `lift_result` promotes `Result<A,E>` to `OptionT<A,E>` — a failing result becomes `Err`. `lift_option` promotes `Option<A>` — a missing option becomes `Ok(None)`. The type alias makes the nested type readable.

## OCaml Approach

OCaml's `OptionT` module: `type ('a, 'e) t = ('a option, 'e) result`. The bind: `let bind ma f = match ma with Error e -> Error e | Ok None -> Ok None | Ok (Some a) -> f a`. `lift_result r = Result.map Option.some r`. `lift_option o = Ok o`. OCaml module functors parameterize over the base monad: `module OptionT(M: MONAD) = struct ...` generates the transformer for any monad M. This generality requires HKTs, which OCaml achieves through module functors.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Type | `type OptionT<A,E> = Result<Option<A>,E>` | Algebraic type or module type |
| Generic transformer | Not expressible (no HKT) | Module functor `OptionT(M: MONAD)` |
| Bind | Free function `bind_opt` | Method or functor-generated |
| `lift_result` | `r.map(Some)` | `Result.map Option.some r` |
| Three-way match | `match ma { Err, Ok(None), Ok(Some) }` | Same |
| Usage | Type alias + functions | Module with `t`, `bind`, etc. |

## Exercises

1. Implement `ResultT<A, E, W> = Writer<Result<A, E>, W>` — a computation that can fail AND accumulates a log.
2. Use `OptionT<A, E>` to implement a database lookup that distinguishes "not found" from "database error."
3. Implement `sequence_opt(ops: Vec<OptionT<A, E>>) -> OptionT<Vec<A>, E>` that collects all results if none are absent or erroneous.
4. Compare explicit nesting (`Result<Option<A>, E>` with manual match) vs. OptionT's `bind_opt` for a multi-step pipeline.
5. Add a `ReaderT<R, A, E> = Reader<R, Result<A, E>>` and implement its bind operation.
