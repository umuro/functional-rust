# OCaml vs Rust: Flattening Nested Lists

## Side-by-Side Comparison

### OCaml (Recursive ADT)
```ocaml
type 'a node =
  | One of 'a
  | Many of 'a node list

let flatten lst =
  let rec aux acc = function
    | [] -> acc
    | One x :: t -> aux (x :: acc) t
    | Many xs :: t -> aux (aux acc xs) t
  in
  List.rev (aux [] lst)
```

### Rust (Recursive Enum)
```rust
#[derive(Debug, PartialEq, Clone)]
enum Node<T> {
    One(T),
    Many(Vec<Node<T>>),
}

fn flatten<T: Clone>(list: &[Node<T>]) -> Vec<T> {
    list.iter()
        .flat_map(|node| match node {
            Node::One(x) => vec![x.clone()],
            Node::Many(xs) => flatten(xs),
        })
        .collect()
}
```

---

## Key Differences

### 1. Type Definition

| Feature | OCaml | Rust |
|---------|-------|------|
| **Syntax** | `type 'a node = \| One of 'a \| Many of 'a node list` | `enum Node<T> { One(T), Many(Vec<Node<T>>) }` |
| **Recursive reference** | Automatic in variant | Requires `Vec` (heap allocation) |
| **Type parameter** | `'a` (implicit) | `<T>` (explicit) |
| **Pattern matching** | `\| One of 'a` | `Node::One(x)` |

**OCaml:**
- Variants are first-class
- Recursive types just work
- Type inference figures out `'a`

**Rust:**
- Enums are namespaced (`Node::One`)
- Recursive types need indirection (Vec/Box)
- Type parameters must be declared

### 2. Recursion Strategy

| Approach | OCaml | Rust |
|----------|-------|------|
| **Style** | Tail-recursive with accumulator | Iterator + `flat_map` |
| **Stack usage** | O(1) with tail calls | O(depth) without TCO guarantee |
| **Allocation** | List consing (structural sharing) | Vec growth (may reallocate) |
| **Reversal** | Needed (`List.rev` at end) | Not needed (iterator order preserved) |

**OCaml approach:**
```ocaml
let rec aux acc = function
  | [] -> acc
  | One x :: t -> aux (x :: acc) t      (* Tail call *)
  | Many xs :: t -> aux (aux acc xs) t  (* Nested recursion, then tail call *)
```

- Builds result backwards via cons (`x :: acc`)
- Reverses at the end for correct order
- OCaml optimizes tail calls → constant stack

**Rust approach:**
```rust
list.iter()
    .flat_map(|node| match node { ... })
    .collect()
```

- Uses iterators (lazy, composable)
- `flat_map` handles nested structure
- `collect()` builds final Vec
- No explicit recursion in client code

### 3. Pattern Matching

**OCaml:**
```ocaml
match node with
| One x -> ...
| Many xs -> ...
```

- Variants match directly
- No namespace qualification
- Type inference knows it's `'a node`

**Rust:**
```rust
match node {
    Node::One(x) => ...,
    Node::Many(xs) => ...,
}
```

- Must qualify enum variants (`Node::`)
- Exhaustiveness checked at compile time
- Type must be known (no inference across match)

### 4. Memory Model

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Nesting** | Variants inline (cons cells) | `Vec` heap-allocated |
| **Sharing** | Immutable sharing possible | Ownership prevents sharing |
| **Cloning** | Cheap (structural sharing) | Explicit (deep copy) |
| **Lifetime** | GC manages | Borrow checker enforces |

**OCaml:**
```ocaml
let shared = Many [One 1; One 2]
let list1 = [shared; One 3]
let list2 = [shared; One 4]  (* Reuses shared structure *)
```

Structural sharing is safe (immutable data).

**Rust:**
```rust
let shared = Node::Many(vec![Node::One(1), Node::One(2)]);
let list1 = vec![shared.clone(), Node::One(3)];
let list2 = vec![shared.clone(), Node::One(4)];  // Must clone
```

Ownership prevents aliasing → need explicit clones.

---

## When to Use Each Approach

### Use OCaml-style (recursive with accumulator)

```rust
fn flatten_recursive<T: Clone>(list: &[Node<T>]) -> Vec<T> {
    fn aux<T: Clone>(mut acc: Vec<T>, list: &[Node<T>]) -> Vec<T> {
        match list {
            [] => {
                acc.reverse();  // OCaml does List.rev
                acc
            },
            [Node::One(x), rest @ ..] => {
                acc.push(x.clone());
                aux(acc, rest)
            },
            [Node::Many(xs), rest @ ..] => {
                let acc = aux(acc, xs);  // Recurse into nested
                aux(acc, rest)           // Then continue
            },
        }
    }
    aux(Vec::new(), list)
}
```

**When:**
- Educational purposes (learning FP patterns)
- Deep nesting (avoid stack overflow concerns)
- Need precise control over allocation

**Trade-offs:**
- More verbose than iterator version
- Rust doesn't guarantee TCO (may still overflow)
- Less idiomatic Rust

### Use Rust-style (iterators + flat_map)

```rust
fn flatten<T: Clone>(list: &[Node<T>]) -> Vec<T> {
    list.iter()
        .flat_map(|node| match node {
            Node::One(x) => vec![x.clone()],
            Node::Many(xs) => flatten(xs),
        })
        .collect()
}
```

**When:**
- Writing production Rust
- Want composable transformations
- Prefer declarative style
- Leverage zero-cost abstractions

**Trade-offs:**
- Recursive calls not tail-optimized
- More allocations (each `vec![x.clone()]`)
- But: compiler can optimize iterator chains

---

## Advanced: Zero-Allocation Version

**Using iterators without intermediate Vecs:**

```rust
use std::iter::once;

fn flatten_iter<T: Clone>(list: &[Node<T>]) -> impl Iterator<Item = T> + '_ {
    list.iter().flat_map(|node| match node {
        Node::One(x) => Box::new(once(x.clone())) as Box<dyn Iterator<Item = T>>,
        Node::Many(xs) => Box::new(flatten_iter(xs)) as Box<dyn Iterator<Item = T>>,
    })
}

// Usage
let result: Vec<_> = flatten_iter(&list).collect();
```

**Benefits:**
- Lazy evaluation (doesn't build intermediate Vecs)
- Can short-circuit (`.take(10)`)
- Composable with other iterator adapters

**Trade-offs:**
- Box<dyn Iterator> has allocation + vtable overhead
- More complex type signatures
- Lifetime management required

---

## Type System Comparison

### OCaml
```ocaml
type 'a node =
  | One of 'a
  | Many of 'a node list

val flatten : 'a node list -> 'a list
```

- Polymorphic automatically
- No explicit trait bounds
- Type inference fills in `'a`

### Rust
```rust
enum Node<T> {
    One(T),
    Many(Vec<Node<T>>),
}

fn flatten<T: Clone>(list: &[Node<T>]) -> Vec<T>
```

- `<T>` explicit type parameter
- `T: Clone` trait bound (because we call `.clone()`)
- Lifetimes implicit here (no references escaping)

**Why `Clone` bound?**

If we return `Vec<&T>` instead:
```rust
fn flatten<'a, T>(list: &'a [Node<T>]) -> Vec<&'a T>
```

No `Clone` needed! Returns references into original data.

---

## Testing Strategies

### OCaml
```ocaml
let () =
  assert (flatten [One 1; Many [One 2; Many [One 3; One 4]]; One 5] 
          = [1; 2; 3; 4; 5]);
  assert (flatten [] = []);
  assert (flatten [One 1] = [1]);
  print_endline "✓ All tests passed"
```

- Assertions in module initialization
- Run immediately when loaded

### Rust
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use Node::*;

    #[test]
    fn test_flatten() {
        assert_eq!(
            flatten(&[One(1), Many(vec![One(2), Many(vec![One(3), One(4)])]), One(5)]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_empty() {
        let empty: Vec<Node<i32>> = vec![];
        assert_eq!(flatten(&empty), Vec::<i32>::new());
    }
}
```

- `#[test]` attribute marks test functions
- Run with `cargo test`
- Type annotations needed for empty cases

---

## Compilation & Performance

### OCaml
```bash
ocamlopt -o flatten example.ml
./flatten
```

- Compiles to native code
- Tail-call optimization guaranteed
- GC handles memory

**Performance:**
- O(n) time (each element visited once)
- O(d) stack (depth of nesting) but tail-optimized
- Structural sharing minimizes allocation

### Rust
```bash
rustc example.rs
./example

# Or with Cargo
cargo build --release
cargo test --release
```

- Compiles to native code
- No TCO guarantee (but LLVM may optimize)
- No GC (deterministic deallocation)

**Performance:**
- O(n) time
- O(d) stack (recursion depth)
- Iterator chains often optimize to tight loops

**Benchmarking tip:** Use `criterion` crate for micro-benchmarks.

---

## Takeaways

1. **Type definition:** Both support recursive algebraic types, Rust requires explicit indirection (Vec/Box)

2. **Recursion style:** OCaml favors tail-recursive accumulators. Rust favors iterators.

3. **Memory model:** OCaml's GC enables structural sharing. Rust's ownership requires cloning.

4. **Idioms:** Write OCaml-style for education, write Rust-style for production.

5. **Performance:** Both compile to fast native code. Rust gives more control, OCaml gives more guarantees (TCO).

**Philosophy:**
- OCaml: "Define the structure, recurse naturally, trust the optimizer."
- Rust: "Compose iterators, let the compiler optimize, own your data."

Both are correct. Choose based on context.
