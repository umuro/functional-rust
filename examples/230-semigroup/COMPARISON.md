# OCaml vs Rust: Semigroup

## Side-by-Side Code

### OCaml

```ocaml
module type SEMIGROUP = sig
  type t
  val append : t -> t -> t
end

module MinSemigroup : SEMIGROUP with type t = int = struct
  type t = int
  let append = min
end

module FirstSemigroup : SEMIGROUP with type t = string = struct
  type t = string
  let append a _ = a
end

let sconcat (module S : SEMIGROUP) lst =
  match lst with
  | []      -> failwith "sconcat: empty list"
  | x :: xs -> List.fold_left S.append x xs
```

### Rust (idiomatic)

```rust
pub trait Semigroup {
    fn append(self, other: Self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Min(pub i64);

impl Semigroup for Min {
    fn append(self, other: Self) -> Self {
        Min(self.0.min(other.0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct First<T>(pub T);

impl<T> Semigroup for First<T> {
    fn append(self, _other: Self) -> Self { self }
}

pub fn sconcat<S: Semigroup + Clone>(items: &[S]) -> Option<S> {
    let (head, tail) = items.split_first()?;
    Some(tail.iter().cloned().fold(head.clone(), |acc, x| acc.append(x)))
}
```

### Rust (functional/recursive)

```rust
pub fn sconcat_recursive<S: Semigroup + Clone>(items: &[S]) -> Option<S> {
    match items {
        [] => None,
        [x] => Some(x.clone()),
        [head, rest @ ..] =>
            sconcat_recursive(rest).map(|tail| head.clone().append(tail)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Trait/signature | `module type SEMIGROUP` | `trait Semigroup` |
| Operation | `val append : t -> t -> t` | `fn append(self, other: Self) -> Self` |
| Instance selection | First-class module `(module MinSemigroup)` | Generic type parameter `<S: Semigroup>` |
| Reduce function | `sconcat : (module SEMIGROUP) -> 'a list -> 'a` | `fn sconcat<S: Semigroup + Clone>(items: &[S]) -> Option<S>` |
| Empty-list failure | `failwith "sconcat: empty list"` | `Option::None` |

## Key Insights

1. **First-class modules → generics:** OCaml passes modules as values at runtime; Rust resolves the instance at compile time via generics — zero runtime overhead.
2. **Newtypes solve coherence:** Rust cannot have two `Semigroup` impls for `i64` (one for Min, one for Max). Newtypes (`Min(i64)`, `Max(i64)`) give each instance a distinct type, making both valid and co-existing.
3. **`Option` instead of exceptions:** OCaml's `failwith` for empty input is a runtime panic; Rust's `Option<S>` return type forces callers to handle the empty case at compile time.
4. **Associativity enables both fold directions:** The iterative (`fold_left`) and recursive (right-associative) versions of `sconcat` must agree by the semigroup law — the test `test_sconcat_and_recursive_agree` verifies this directly.
5. **Ownership semantics:** `append(self, other: Self)` consumes both arguments. This is natural for newtypes wrapping `Copy` types (`Min`, `Max`) and forces explicit `Clone` when operating on slices.

## When to Use Each Style

**Use idiomatic Rust when:** reducing a slice in a hot path — `split_first` + `fold` is a single iterator pass with no recursion overhead.

**Use recursive Rust when:** mirroring an OCaml original for pedagogical comparison, or when the right-associative fold is semantically meaningful (e.g., building a right-associative data structure).
