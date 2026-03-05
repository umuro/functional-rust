# Comparison: Applicative Functor Basics

## Apply Operation

**OCaml:**
```ocaml
let apply mf mx = match mf with
  | Nothing -> Nothing
  | Just f -> map f mx

let ( <*> ) = apply

(* Usage: pure add <*> Just 3 <*> Just 4 = Just 7 *)
```

**Rust:**
```rust
impl<F> Maybe<F> {
    fn apply<A, B>(self, ma: Maybe<A>) -> Maybe<B>
    where F: FnOnce(A) -> B {
        match (self, ma) {
            (Maybe::Just(f), Maybe::Just(a)) => Maybe::Just(f(a)),
            _ => Maybe::Nothing,
        }
    }
}
```

## Lifting Multi-Argument Functions

**OCaml:**
```ocaml
(* Currying makes this elegant *)
let lift2 f a b = (pure f) <*> a <*> b
let result = lift2 (+) (Just 10) (Just 20)  (* Just 30 *)
```

**Rust:**
```rust
// No currying — take multi-arg closure directly
fn lift2_simple<A, B, C, F: FnOnce(A, B) -> C>(
    f: F, a: Maybe<A>, b: Maybe<B>,
) -> Maybe<C> {
    match (a, b) {
        (Maybe::Just(a), Maybe::Just(b)) => Maybe::Just(f(a, b)),
        _ => Maybe::Nothing,
    }
}

let result = lift2_simple(|a, b| a + b, Maybe::Just(10), Maybe::Just(20));
```

## Built-in Applicative in Rust

**Rust (Option::zip):**
```rust
let a = Some(3);
let b = Some(4);
let result = a.zip(b).map(|(a, b)| a + b); // Some(7)
```
