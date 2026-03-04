# OCaml vs Rust: Continuation-Passing Style

## Side-by-Side Code

### OCaml

```ocaml
(* CPS factorial — tail-recursive *)
let factorial_cps n =
  let rec go n k =
    if n = 0 then k 1
    else go (n - 1) (fun result -> k (n * result))
  in
  go n Fun.id

(* CPS tree sum *)
type 'a tree = Leaf of 'a | Node of 'a tree * 'a tree

let sum_cps t =
  let rec go t k = match t with
    | Leaf x -> k x
    | Node (l, r) -> go l (fun sl -> go r (fun sr -> k (sl + sr)))
  in go t Fun.id
```

### Rust (idiomatic — direct style for comparison)

```rust
pub fn factorial_direct(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial_direct(n - 1)
    }
}
```

### Rust (CPS — continuation-passing style)

```rust
fn factorial_go(n: u64, k: Box<dyn FnOnce(u64) -> u64>) -> u64 {
    if n == 0 {
        k(1)
    } else {
        factorial_go(n - 1, Box::new(move |result| k(n * result)))
    }
}

pub fn factorial_cps(n: u64) -> u64 {
    factorial_go(n, Box::new(|x| x))
}

fn sum_go<'a>(t: &'a Tree<i64>, k: Box<dyn FnOnce(i64) -> i64 + 'a>) -> i64 {
    match t {
        Tree::Leaf(x) => k(*x),
        Tree::Node(l, r) => sum_go(
            l,
            Box::new(move |sl| sum_go(r, Box::new(move |sr| k(sl + sr)))),
        ),
    }
}

pub fn sum_cps(t: &Tree<i64>) -> i64 {
    sum_go(t, Box::new(|x| x))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Continuation type | `'a -> 'b` (any function) | `Box<dyn FnOnce(T) -> R>` |
| Identity continuation | `Fun.id` | `Box::new(\|x\| x)` |
| Tree type | `'a tree` (algebraic) | `enum Tree<T>` with `Box<Tree<T>>` children |
| CPS factorial | `val factorial_cps : int -> int` | `fn factorial_cps(n: u64) -> u64` |
| CPS tree sum | `val sum_cps : int tree -> int` | `fn sum_cps(t: &Tree<i64>) -> i64` |

## Key Insights

1. **Why `FnOnce` not `Fn`:** Each CPS continuation is consumed exactly once — it either produces the final answer or is wrapped inside a new closure and passed along. Using `Fn` would require the closure to be callable multiple times, but the captured inner continuation (`k`) is moved out on the first (and only) call. `FnOnce` models this precisely.

2. **Type erasure with `Box<dyn ...>`:** In OCaml, the type checker unifies continuation types polymorphically — every `go ... (fun x -> ...)` just produces `'a -> 'b`. In Rust, each new wrapping creates a *distinct* concrete closure type. Because these types are infinitely nested and heterogeneous, Rust cannot represent the type statically; `Box<dyn FnOnce>` erases the concrete type at a small runtime cost (one heap allocation per CPS step, one vtable dispatch per call).

3. **No TCO guarantee:** OCaml's compiler performs self-tail-call optimisation, so `go (n-1) (fun ...)` does not grow the call stack — it reuses the current frame. Rust provides no such guarantee. The CPS transform makes the algorithm *structurally* tail-recursive (the recursive call is syntactically last), but LLVM may or may not eliminate the frame. For true stack safety in Rust, use a trampoline (return a thunk and drive it from a loop) or an explicit accumulator.

4. **Lifetimes on continuations:** `sum_go` borrows tree nodes via `&'a Tree<i64>`. The continuation chain captures these references, so the boxed closure must carry the lifetime: `Box<dyn FnOnce(i64) -> i64 + 'a>`. This makes the borrow checker verify that tree nodes outlive their continuations — a guarantee OCaml's GC provides implicitly.

5. **`move` closures for ownership transfer:** Each `Box::new(move |result| k(n * result))` moves `k` and `n` into the new closure. Without `move`, `k` would be borrowed, and since the outer function returns before the closure runs, the borrow would dangle. Rust's ownership rules enforce this discipline at compile time.

## When to Use Each Style

**Use direct style when:** The recursion depth is bounded and small (e.g., tree height < few thousand), or when clarity matters more than tail-call guarantees. Most Rust code uses direct style.

**Use CPS in Rust when:** You need to explicitly model asynchronous callbacks, build interpreter evaluation loops, or demonstrate the CPS transform for educational purposes. For stack safety, prefer `Vec`-based explicit stacks or the `stacker` crate over CPS.
