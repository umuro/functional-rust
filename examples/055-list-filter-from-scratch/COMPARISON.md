# Comparison: List Filter from Scratch

## OCaml — recursive pattern match

```ocaml
let rec filter p = function
  | [] -> []
  | h :: t -> if p h then h :: filter p t else filter p t
```

## Rust — iterator (idiomatic)

```rust
pub fn filter<T: Clone, P: Fn(&T) -> bool>(list: &[T], p: P) -> Vec<T> {
    list.iter().filter(|x| p(x)).cloned().collect()
}
```

## Rust — recursive (structural translation)

```rust
pub fn filter_rec<T: Clone, P: Fn(&T) -> bool>(list: &[T], p: P) -> Vec<T> {
    fn go<T: Clone>(list: &[T], p: &dyn Fn(&T) -> bool) -> Vec<T> {
        match list {
            [] => vec![],
            [head, tail @ ..] => {
                let mut rest = go(tail, p);
                if p(head) {
                    let mut result = vec![head.clone()];
                    result.append(&mut rest);
                    result
                } else {
                    rest
                }
            }
        }
    }
    go(list, &p)
}
```

## Rust — fold

```rust
pub fn filter_fold<T: Clone, P: Fn(&T) -> bool>(list: &[T], p: P) -> Vec<T> {
    list.iter().fold(Vec::new(), |mut acc, x| {
        if p(x) { acc.push(x.clone()); }
        acc
    })
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Base case | `\| [] -> []` | `[] => vec![]` |
| Recursive case | `\| h :: t -> ...` | `[head, tail @ ..] => ...` |
| Predicate type | `'a -> bool` | `Fn(&T) -> bool` |
| Partial application | `let evens = filter f` | closure wrapping |
| Recursion safety | structural, GC-managed | needs `&dyn Fn` to avoid monomorphization loop |
| Iterator style | `List.filter` in stdlib | `Iterator::filter` in stdlib |

## Type Signatures

- OCaml: `val filter : ('a -> bool) -> 'a list -> 'a list`
- Rust: `fn filter<T: Clone, P: Fn(&T) -> bool>(list: &[T], p: P) -> Vec<T>`

## Takeaways

1. Rust's `[head, tail @ ..]` slice pattern is the structural analogue of OCaml's `h :: t` — same idea, different syntax.
2. Passing a generic `P: Fn` through recursive calls creates an infinite chain of wrapper types at compile time; the `&dyn Fn` inner helper erases the type, breaking the cycle.
3. The iterator version (`filter`) is the most idiomatic and is what `Iterator::filter` does — studying the from-scratch versions illuminates the standard library.
4. All three implementations produce the same result; the fold form is the most allocation-friendly for in-order traversal.
5. OCaml partial application (`let evens = filter f`) is concise; Rust achieves the same by capturing the predicate in a closure.
