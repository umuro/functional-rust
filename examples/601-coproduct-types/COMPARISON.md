# Coproduct Types

A coproduct is a "tagged union" or "sum type" - a value that is one of several possibilities.

## Rust
```rust
enum Either<A, B> { Left(A), Right(B) }
```

## OCaml
```ocaml
type ('a, 'b) either = Left of 'a | Right of 'b
```

Either is the canonical two-way coproduct.
Rust's Result<T, E> is essentially Either<T, E> with different names.
