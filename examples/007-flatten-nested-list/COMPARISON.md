# Comparison: Flatten Nested List

## OCaml — Recursive with accumulator

```ocaml
type 'a node = One of 'a | Many of 'a node list

let flatten lst =
  let rec aux acc = function
    | [] -> acc
    | One x :: t -> aux (x :: acc) t
    | Many xs :: t -> aux (aux acc xs) t
  in
  List.rev (aux [] lst)
```

## Rust — Idiomatic (flat_map)

```rust
pub fn flatten<T: Clone>(list: &[Node<T>]) -> Vec<T> {
    list.iter()
        .flat_map(|node| match node {
            Node::One(x) => vec![x.clone()],
            Node::Many(xs) => flatten(xs),
        })
        .collect()
}
```

## Rust — Stack-based (no recursion)

```rust
pub fn flatten_stack<T: Clone>(list: &[Node<T>]) -> Vec<T> {
    let mut result = Vec::new();
    let mut stack: Vec<&Node<T>> = list.iter().rev().collect();
    while let Some(node) = stack.pop() {
        match node {
            Node::One(x) => result.push(x.clone()),
            Node::Many(xs) => {
                for child in xs.iter().rev() {
                    stack.push(child);
                }
            }
        }
    }
    result
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type definition | `type 'a node = One of 'a \| Many of 'a node list` | `enum Node<T> { One(T), Many(Vec<Node<T>>) }` |
| Recursion | Tail-recursive with accumulator | Stack-based iteration (safer) |
| Memory | GC-managed cons cells | Owned `Vec` on heap |
| Copying | Implicit (GC) | Explicit `Clone` trait bound |
| Ownership | Shared by default | Must choose: borrow (`&`) or consume |

## Takeaways

1. Rust enums are algebraic data types — same expressive power as OCaml variants
2. Without guaranteed TCO, prefer explicit stacks for potentially deep recursion in Rust
3. The ownership system forces you to decide: clone data or consume it — OCaml hides this choice
4. `flat_map` gives the most readable recursive solution but allocates intermediate vectors
5. The consuming version (`flatten_owned`) is uniquely Rust — it's impossible in GC languages where data is always shared
