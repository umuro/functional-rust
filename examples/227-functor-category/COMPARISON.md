# OCaml vs Rust: Functor Category — Natural Transformations

## Side-by-Side Code

### OCaml

```ocaml
(* Functor interface using module signature — HKT via type 'a t *)
module type FUNCTOR = sig
  type 'a t
  val map : ('a -> 'b) -> 'a t -> 'b t
end

(* Natural transformation: list → option (take head) *)
let list_to_option : 'a list -> 'a option = function
  | []     -> None
  | x :: _ -> Some x

(* Naturality: map f . nat = nat . map f *)
let check_naturality () =
  let f x = x * 2 in
  let lst = [1; 2; 3] in
  let lhs = list_to_option (List.map f lst) in
  let rhs = Option.map f (list_to_option lst) in
  assert (lhs = rhs)
```

### Rust (idiomatic)

```rust
// Natural transformation: slice → Option, borrows in/out (no allocation)
pub fn list_to_option<T>(list: &[T]) -> Option<&T> {
    list.first()
}

// Natural transformation: Option → Vec
pub fn option_to_list<T>(opt: Option<T>) -> Vec<T> {
    match opt {
        None => vec![],
        Some(x) => vec![x],
    }
}
```

### Rust (functional/recursive)

```rust
// Same natural transformation, spelled out as a recursive pattern match
// mirroring the OCaml `function | [] -> None | x :: _ -> Some x`
pub fn list_to_option_rec<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,
        [head, ..] => Some(head),
    }
}

// Naturality condition as a typed, generic predicate
pub fn naturality_holds<T, U, F>(list: &[T], f: F) -> bool
where
    T: Clone,
    U: PartialEq,
    F: Fn(T) -> U,
{
    let mapped: Vec<U> = list.iter().cloned().map(&f).collect();
    let lhs = mapped.first();
    let rhs = list.first().cloned().map(f);
    lhs == rhs.as_ref()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Functor interface | `module type FUNCTOR` with `type 'a t` | No direct equivalent (no HKT in stable Rust) |
| List type | `'a list` | `&[T]` (borrowed slice) |
| Natural transformation | `'a list -> 'a option` | `fn list_to_option<T>(list: &[T]) -> Option<&T>` |
| Option type | `'a option` | `Option<T>` |
| Map over list | `List.map f lst` | `list.iter().cloned().map(f).collect::<Vec<_>>()` |
| Map over option | `Option.map f opt` | `opt.map(f)` |
| Naturality assertion | `assert (lhs = rhs)` | `naturality_holds(list, f)` — returns `bool` |

## Key Insights

1. **HKT gap:** OCaml can express `FUNCTOR` as a module type with `type 'a t` — a higher-kinded type. Rust has no equivalent in stable code, so the functor concept remains implicit in how `Vec<T>` and `Option<T>` both support `.map()`.

2. **Ownership shapes signatures:** OCaml's `list_to_option` takes and returns values freely. In Rust, the idiomatic version takes a `&[T]` (a borrow) and returns `Option<&T>` — a reference into the input — avoiding any heap allocation for the common case.

3. **Naturality as a first-class predicate:** OCaml verifies naturality with an `assert` at runtime. Rust encodes it as `naturality_holds<T, U, F>` — a generic function whose type bounds (`T: Clone, U: PartialEq, F: Fn(T) -> U`) document exactly what the condition requires, making the proof obligation visible in the type system.

4. **Pattern matching parity:** The recursive Rust version (`match list { [] => None, [head, ..] => Some(head) }`) is almost a direct transliteration of OCaml's `function | [] -> None | x :: _ -> Some x`, showing how slice patterns bridge the two languages.

5. **`as_ref()` for cross-ownership comparison:** To compare `Option<&U>` (lhs) with `Option<U>` (rhs) in `naturality_holds`, Rust requires `rhs.as_ref()` to get `Option<&U>`. OCaml performs structural equality without worrying about ownership, illustrating how Rust's ownership model adds small but necessary boilerplate at comparison sites.

## When to Use Each Style

**Use idiomatic Rust (`list.first()`)** when you need a fast, zero-copy check and the caller already holds the slice — the borrow is cheap and the API is maximally general.

**Use recursive Rust** when teaching the OCaml parallel explicitly, or when the pattern-match structure itself is the point (e.g., demonstrating structural recursion or showing the empty/non-empty case split).
