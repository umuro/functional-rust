# OCaml vs Rust: Natural Transformations

## Side-by-Side Code

### OCaml

```ocaml
(* Safe head: list -> option (natural transformation) *)
let safe_head lst = match lst with [] -> None | x :: _ -> Some x

(* Verify naturality: nat(fmap f xs) == fmap f (nat xs) *)
let verify_naturality f nat lst =
  let lhs = nat (List.map f lst) in
  let rhs = Option.map f (nat lst) in
  lhs = rhs

(* Composition: list -[safe_head]-> option -[option_to_list]-> list *)
let option_to_list o = match o with None -> [] | Some x -> [x]
let nat_composed lst = option_to_list (safe_head lst)
```

### Rust (idiomatic)

```rust
pub fn safe_head<T: Clone>(list: &[T]) -> Option<T> {
    list.first().cloned()
}

pub fn verify_naturality<T, U>(
    f: impl Fn(T) -> U,
    nat_t: impl Fn(&[T]) -> Option<T>,
    nat_u: impl Fn(&[U]) -> Option<U>,
    list: &[T],
) -> bool
where
    T: Clone,
    U: PartialEq,
{
    let mapped: Vec<U> = list.iter().map(|x| f(x.clone())).collect();
    let lhs = nat_u(&mapped);
    let rhs = nat_t(list).map(f);
    lhs == rhs
}
```

### Rust (functional/recursive)

```rust
pub fn safe_head_recursive<T: Clone>(list: &[T]) -> Option<T> {
    match list {
        [] => None,
        [x, ..] => Some(x.clone()),
    }
}

pub fn option_to_vec<T>(opt: Option<T>) -> Vec<T> {
    match opt {
        None => vec![],
        Some(x) => vec![x],
    }
}

pub fn nat_composed<T: Clone>(list: &[T]) -> Vec<T> {
    option_to_vec(safe_head(list))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Safe head | `val safe_head : 'a list -> 'a option` | `fn safe_head<T: Clone>(list: &[T]) -> Option<T>` |
| Option to list | `val option_to_list : 'a option -> 'a list` | `fn option_to_vec<T>(opt: Option<T>) -> Vec<T>` |
| Naturality verifier | `('a -> 'b) -> ('a list -> 'a option) -> 'a list -> bool` | `(T→U, Fn(&[T])→Option<T>, Fn(&[U])→Option<U>, &[T]) → bool` |
| Composed nat trans | `'a list -> 'a list` | `fn nat_composed<T: Clone>(list: &[T]) -> Vec<T>` |

## Key Insights

1. **Parametric naturality:** In both languages, a polymorphic function that treats its
   type parameter uniformly (no `Typeable`/`Any` tricks) is automatically natural. Rust's
   generics enforce this structurally.

2. **Rank-2 types:** OCaml accepts a single polymorphic `nat` argument. Rust requires two
   monomorphized copies (`nat_t` and `nat_u`), since Rust has no rank-2 type polymorphism.
   The compiler instantiates the same generic function at both types at call sites.

3. **Ownership and cloning:** OCaml's garbage collector shares values freely. Rust's
   `T: Clone` bound makes the copy cost explicit — `.cloned()` on slices allocates owned
   values so we can return them without dangling references.

4. **The naturality square:** The commuting condition `nat(fmap f xs) == fmap f (nat xs)`
   means applying the morphism before or after the nat transformation yields the same result.
   This is what makes a nat transformation "structure preserving" across the functor category.

5. **Composition:** Natural transformations compose component-wise. `option_to_vec ∘ safe_head`
   yields another valid nat transformation `[T] → Vec<T>`, which is exactly `nat_composed`.
   The type system ensures the composition is well-formed.

## When to Use Each Style

**Use idiomatic Rust when:** Calling library methods like `.first()`, `.last()`, or
`.cloned()` on slices — they compose cleanly and communicate intent directly.

**Use recursive Rust when:** Teaching the OCaml parallel explicitly, or when the
structural decomposition of the input (empty vs. cons) is the central learning point.
