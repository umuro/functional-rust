# OCaml vs Rust: List.filter — Select Elements by Predicate

## Side-by-Side Code

### OCaml

```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds  = List.filter (fun x -> x mod 2 <> 0) numbers
```

### Rust (idiomatic — borrowing)

```rust
pub fn filter<T, F>(items: &[T], predicate: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| predicate(x)).collect()
}

let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
let evens: Vec<&i32> = filter(&numbers, |x| x % 2 == 0);
let odds:  Vec<&i32> = filter(&numbers, |x| x % 2 != 0);
```

### Rust (idiomatic — owned)

```rust
pub fn filter_owned<T, F>(items: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    items.into_iter().filter(|x| predicate(x)).collect()
}
```

### Rust (functional/recursive)

```rust
pub fn filter_recursive<'a, T, F>(items: &'a [T], predicate: &F) -> Vec<&'a T>
where
    F: Fn(&T) -> bool,
{
    match items {
        [] => vec![],
        [head, tail @ ..] => {
            let mut rest = filter_recursive(tail, predicate);
            if predicate(head) {
                let mut result = vec![head];
                result.append(&mut rest);
                result
            } else {
                rest
            }
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val filter : ('a -> bool) -> 'a list -> 'a list` | `fn filter<T, F>(items: &[T], predicate: F) -> Vec<&T>` |
| List/slice type | `'a list` | `&[T]` (borrowed slice) or `Vec<T>` (owned) |
| Predicate type | `'a -> bool` | `F: Fn(&T) -> bool` |
| Return type | `'a list` | `Vec<&T>` or `Vec<T>` |
| Polymorphism | Type inference + structural | Generic monomorphization |

## Key Insights

1. **Direct functional parallel:** `List.filter pred lst` in OCaml maps precisely to `lst.iter().filter(pred).collect()` in Rust — both apply a boolean predicate and gather matching elements.

2. **Two Rust flavors:** Rust distinguishes between filtering borrowed data (`&[T]` → `Vec<&T>`) and owned data (`Vec<T>` → `Vec<T>`). OCaml has no such distinction because its values are always behind a pointer and garbage collected.

3. **Lifetime annotation in recursive form:** The recursive Rust version must carry a named lifetime `'a` to prove that the returned references come from `items`, not from the `predicate`. This explicitness is absent in OCaml, where the runtime handles reference lifetimes.

4. **Predicate as generic vs value:** OCaml predicates are ordinary values of type `'a -> bool`. Rust predicates are generic type parameters constrained by `Fn(&T) -> bool`, which allows the compiler to monomorphize them for zero-cost dispatch — equivalent to C++ templates, but checked at the call site.

5. **Immutability and allocation:** Both languages produce a new collection without mutating the input. Rust makes this explicit via the type system: `&[T]` cannot be modified, and `into_iter()` consumes the source `Vec`.

## When to Use Each Style

**Use idiomatic Rust (`.iter().filter()`) when:** filtering borrowed data or when you need to chain further iterator operations before collecting — it avoids intermediate allocations.

**Use `filter_owned` (`into_iter().filter()`) when:** you no longer need the original `Vec` and want to avoid double allocation; the source is consumed and a new `Vec` is returned directly.

**Use recursive Rust when:** teaching the OCaml → Rust translation explicitly, or when the problem naturally recurses (e.g., filtering a custom linked-list type where slice patterns don't apply).
