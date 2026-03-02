# Comparison: List Map from Scratch

## OCaml — recursive definition

```ocaml
let rec map f = function
  | [] -> []
  | h :: t -> let h' = f h in h' :: map f t
```

## Rust — iterator (idiomatic)

```rust
pub fn map<A, B, F: Fn(&A) -> B>(list: &[A], f: F) -> Vec<B> {
    list.iter().map(f).collect()
}
```

## Rust — recursive (mirrors OCaml)

```rust
pub fn map_recursive<A, B, F: Fn(&A) -> B>(list: &[A], f: F) -> Vec<B> {
    match list {
        [] => vec![],
        [head, tail @ ..] => {
            let mut result = vec![f(head)];
            result.extend(map_recursive(tail, f));
            result
        }
    }
}
```

## Rust — fold

```rust
pub fn map_fold<A, B, F: Fn(&A) -> B>(list: &[A], f: F) -> Vec<B> {
    list.iter().fold(Vec::new(), |mut acc, x| {
        acc.push(f(x));
        acc
    })
}
```

## Partial application comparison

| OCaml | Rust |
|-------|------|
| `let add1 = map (fun x -> x + 1)` | `fn add1(list: &[i32]) -> Vec<i32> { map(list, \|x\| x + 1) }` |
| `let double = map (fun x -> x * 2)` | `fn double(list: &[i32]) -> Vec<i32> { map(list, \|x\| x * 2) }` |
| `let to_string = map string_of_int` | `fn to_string(list: &[i32]) -> Vec<String> { map(list, \|x\| x.to_string()) }` |

OCaml curries automatically; Rust requires explicit wrapping because `map` takes two arguments rather than returning a closure.

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type signature | `('a -> 'b) -> 'a list -> 'b list` | `fn<A,B,F:Fn(&A)->B>(&[A], F) -> Vec<B>` |
| Pattern match | `\| [] -> [] \| h :: t -> ...` | `[] => vec![] \| [head, tail @ ..] => ...` |
| Partial application | Native currying | Closure wrapping |
| Memory model | GC-managed cons cells | Contiguous `Vec<B>` on heap |
| Tail recursion | Not tail-recursive (OCaml TCO not applied here) | Not tail-recursive (use iterator version) |
| Idiomatic style | Recursive with pattern match | `iter().map().collect()` |

## Takeaways

- `map` is the canonical abstraction for element-wise list transformation
- Rust's iterator `map` adapter and OCaml's `List.map` have the same semantics; both are lazy/eager respectively
- The recursive Rust version uses slice patterns (`[head, tail @ ..]`) which are the closest structural analogue to OCaml's cons patterns
- Fold proves `map` is a specialisation: every `map` can be rewritten as a `fold`
