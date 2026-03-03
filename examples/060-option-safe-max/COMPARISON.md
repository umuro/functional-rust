# Comparison: Option Type — Safe List Maximum — OCaml vs Rust

## OCaml

```ocaml
let rec list_max = function
  | []     -> None
  | h :: t ->
    begin match list_max t with
    | None   -> Some h
    | Some m -> Some (max h m)
    end

let safe_head = function
  | []     -> None
  | h :: _ -> Some h

let option_map f = function
  | None   -> None
  | Some x -> Some (f x)
```

## Rust — Idiomatic

```rust
pub fn list_max_idiomatic(xs: &[i32]) -> Option<i32> {
    xs.iter().copied().max()
}

pub fn safe_head_idiomatic(xs: &[i32]) -> Option<i32> {
    xs.first().copied()
}

pub fn double_max_idiomatic(xs: &[i32]) -> Option<i32> {
    list_max_idiomatic(xs).map(|x| x * 2)
}
```

## Rust — Functional (Recursive)

```rust
pub fn list_max_recursive(xs: &[i32]) -> Option<i32> {
    match xs {
        [] => None,
        [head, tail @ ..] => match list_max_recursive(tail) {
            None => Some(*head),
            Some(m) => Some(if *head > m { *head } else { m }),
        },
    }
}
```

## Rust — Fold-based

```rust
pub fn list_max_fold(xs: &[i32]) -> Option<i32> {
    let (&first, rest) = xs.split_first()?;
    Some(rest.iter().fold(first, |acc, &x| acc.max(x)))
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Option type | `'a option = None \| Some of 'a` | `Option<T> = None \| Some(T)` |
| Pattern matching | `match ... with None -> ... \| Some x -> ...` | `match ... { None => ..., Some(x) => ... }` |
| Map over option | Custom `option_map` or `Option.map` | Built-in `Option::map` |
| Chaining | `Option.bind` | `.and_then()` or `?` operator |
| Max of iterator | Manual recursion | `iter().max()` returns `Option` |
| Head of list | Manual pattern match | `slice.first()` returns `Option<&T>` |
| Copy from ref | Not needed (no ref distinction) | `.copied()` converts `Option<&T>` → `Option<T>` |

## Type Signatures Explained

**OCaml:** `val list_max : 'a list -> 'a option` — polymorphic over any ordered type (uses structural comparison)

**Rust:** `fn list_max_idiomatic(xs: &[i32]) -> Option<i32>` — concrete type. Generic version would be `fn list_max<T: Ord + Copy>(xs: &[T]) -> Option<T>` requiring explicit trait bounds.

## Takeaways

1. **Option is the same idea** in both languages — `Some`/`None` with exhaustive matching, no nulls
2. **Rust's Option is richer** — 40+ methods (`map`, `and_then`, `unwrap_or`, `filter`, `zip`, `?`)
3. **The `?` operator** is Rust's killer feature for Option — `xs.split_first()?` propagates `None` elegantly
4. **`.copied()` bridge** is needed because Rust distinguishes `&T` from `T`; OCaml doesn't
5. **Iterator integration** means Rust rarely needs custom `list_max` — `iter().max()` is built-in and returns `Option`
