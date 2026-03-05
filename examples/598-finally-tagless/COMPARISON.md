# OCaml vs Rust: Finally Tagless

## Key Idea

Instead of an ADT for expressions, use a trait/module signature.
Each interpretation implements the trait differently.

## OCaml (Module)
```ocaml
module type ExprAlg = sig
  type repr
  val lit : int -> repr
  val add : repr -> repr -> repr
end
```

## Rust (Trait)
```rust
trait ExprAlg {
    type Repr;
    fn lit(&self, n: i32) -> Self::Repr;
    fn add(&self, a: Self::Repr, b: Self::Repr) -> Self::Repr;
}
```

## Benefits

1. **Extensible** - Add new operations via trait extension
2. **Multiple interpretations** - Eval, pretty-print, optimize
3. **Type-safe** - No runtime pattern matching
