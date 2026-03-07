# OCaml vs Rust: List Filter

## Side-by-Side Code

### OCaml
```ocaml
let rec filter p = function
  | []     -> []
  | h :: t ->
    let t' = filter p t in
    if p h then h :: t' else t'

let is_even x = x mod 2 = 0

let () =
  let nums = [-2; -1; 0; 1; 2; 3; 4] in
  List.iter (Printf.printf "%d ") (filter is_even nums); print_newline ()
```

### Rust (idiomatic)
```rust
pub fn filter<T: Clone>(predicate: fn(&T) -> bool, items: &[T]) -> Vec<T> {
    items.iter().filter(|x| predicate(x)).cloned().collect()
}

pub fn is_even(x: &i32) -> bool {
    x % 2 == 0
}

fn main() {
    let nums = vec![-2, -1, 0, 1, 2, 3, 4];
    let evens = filter(is_even, &nums);
    for n in evens {
        print!("{} ", n);
    }
    println!();
}
```

### Rust (recursive)
```rust
pub fn filter_recursive<T: Clone>(predicate: fn(&T) -> bool, items: &[T]) -> Vec<T> {
    match items {
        [] => vec![],
        [h, rest @ ..] => {
            let mut tail = filter_recursive(predicate, rest);
            if predicate(h) {
                let mut result = vec![h.clone()];
                result.append(&mut tail);
                result
            } else {
                tail
            }
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Filter function | `('a -> bool) -> 'a list -> 'a list` | `fn(fn(&T) -> bool, &[T]) -> Vec<T>` |
| Predicate | `'a -> bool` | `fn(&T) -> bool` |
| List type | `'a list` | `Vec<T>` or `&[T]` |
| Empty list | `[]` | `vec![]` or `[]` (pattern) |
| Cons | `h :: t` | `[h, rest @ ..]` (pattern) |

## Key Insights

1. **Pattern matching equivalence:** OCaml's `h :: t` directly maps to Rust's `[h, rest @ ..]` slice pattern. Both deconstruct lists into head and tail in a single match expression.

2. **Ownership vs. sharing:** OCaml lists are immutable and share structure via references; `h :: t'` reuses memory. Rust vectors must `.clone()` each element because vectors own their data. This is why the idiomatic Rust version collects into `Vec<T>` instead of building a linked list.

3. **Function types as parameters:** OCaml's `'a -> bool` is a universal function type. Rust's `fn(&T) -> bool` is more restrictive—it requires a function pointer, not a closure. For general higher-order functions, Rust would use `impl Fn(&T) -> bool` or `Box<dyn Fn(&T) -> bool>`, but function pointers suffice here.

4. **Idiomatic style divergence:** OCaml recursion is the natural, encouraged pattern. Rust's idiomatic style favors iterators—they compose better, avoid stack depth issues, and integrate with the standard library. Recursion in Rust is a valid but secondary choice.

5. **Predicate binding:** In OCaml, the predicate `p` is bound once at the function's start. In Rust, we pass `predicate: fn(&T) -> bool` explicitly. This makes the dependency clearer and enables partial application via closures (though function pointers don't capture state).

## When to Use Each Style

**Use idiomatic Rust iterator when:**
- You're filtering within a data pipeline (map, filter, fold chains)
- Performance matters (iterators may optimize better and avoid intermediate allocations)
- The predicate is simple or from the standard library (e.g., `is_some`, `is_ok`)

**Use recursive Rust when:**
- Teaching functional programming concepts or comparing directly with OCaml
- The predicate has complex control flow that reads better as pattern matches
- You need to preserve the functional recursion pattern for clarity

**OCaml recursion is always natural because:**
- Immutable data structures encourage recursion
- No stack depth concerns for typical list sizes
- Pattern matching is the primary control flow mechanism
