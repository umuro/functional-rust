# OCaml vs Rust: Rose Tree — Multi-Way Tree with Fold

## Side-by-Side Code

### OCaml
```ocaml
type 'a rose = Rose of 'a * 'a rose list

let rec fold f (Rose (x, children)) =
  f x (List.map (fold f) children)

let size = fold (fun _ sizes -> 1 + List.fold_left (+) 0 sizes)
let depth = fold (fun _ depths -> 1 + List.fold_left max 0 depths)

let to_string = fold (fun x strs ->
  match strs with
  | [] -> x
  | _ -> x ^ "(" ^ String.concat "," strs ^ ")")
```

### Rust (idiomatic)
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Rose<T> {
    pub value: T,
    pub children: Vec<Rose<T>>,
}

impl<T> Rose<T> {
    pub fn fold<R>(&self, f: &dyn Fn(&T, Vec<R>) -> R) -> R {
        let child_results: Vec<R> = self.children.iter().map(|c| c.fold(f)).collect();
        f(&self.value, child_results)
    }
}

pub fn size<T>(tree: &Rose<T>) -> usize {
    tree.fold(&|_, sizes: Vec<usize>| 1 + sizes.iter().sum::<usize>())
}
```

### Rust (functional/recursive preorder)
```rust
pub fn preorder<T: Clone>(tree: &Rose<T>) -> Vec<T> {
    let mut result = vec![tree.value.clone()];
    for child in &tree.children {
        result.extend(preorder(child));
    }
    result
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Rose tree | `'a rose` | `Rose<T>` |
| Fold | `('a -> 'b list -> 'b) -> 'a rose -> 'b` | `fn fold<R>(&self, f: &dyn Fn(&T, Vec<R>) -> R) -> R` |
| Children | `'a rose list` | `Vec<Rose<T>>` |
| Size | `'a rose -> int` | `fn size<T>(tree: &Rose<T>) -> usize` |
| Depth | `'a rose -> int` | `fn depth<T>(tree: &Rose<T>) -> usize` |

## Key Insights

1. **Struct vs variant:** OCaml packs value and children into a single variant constructor; Rust separates them into named struct fields, improving readability
2. **Vec vs list for children:** Rust's Vec gives O(1) random access and better cache locality than OCaml's linked list; both support iteration
3. **Trait objects for recursive HOF:** Rust needs `&dyn Fn` to pass closures through recursive fold calls; OCaml handles this transparently
4. **Partial application gap:** OCaml's `let size = fold (fun ...)` partially applies fold elegantly; Rust needs a standalone function wrapper
5. **Collect pattern:** OCaml's `List.map` naturally maps over children; Rust uses `.iter().map().collect()` — more explicit but equally clear

## When to Use Each Style

**Use fold-based derivation when:** you have multiple operations over the same tree structure — write fold once, derive everything  
**Use direct recursion when:** you need fine-grained control over traversal order or want to short-circuit early
