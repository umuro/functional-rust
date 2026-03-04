# 055: List Filter from Scratch

**Difficulty:** 1  **Level:** Beginner

Implement `filter` from first principles using a predicate function — and derive it from `fold`.

## The Problem This Solves

You have a list of orders and you want only the pending ones. A list of log lines and you want only the errors. A list of numbers and you want only the evens. Without `filter`, you write the same accumulator loop for every predicate, sprinkling `if` guards through your business logic.

`filter` separates the *what to keep* (the predicate) from the *how to keep it* (the traversal). You write `filter(orders, |o| o.status == Pending)` and the loop disappears.

Like `map`, `filter` is derivable from `fold` — which reveals that fold truly is the universal list combinator.

## The Intuition

`filter` is a sieve. You shake the list; elements that pass the predicate fall through, the rest are discarded. The order of survivors is preserved — an important guarantee for correctness.

The predicate is just a function that returns `bool`. Any function. `filter` doesn't care what the test is; it just applies it to each element and decides keep or discard.

## How It Works in Rust

```rust
// Idiomatic: iterator adapter
pub fn filter<T: Clone, P: Fn(&T) -> bool>(list: &[T], p: P) -> Vec<T> {
    list.iter().filter(|x| p(x)).cloned().collect()
    // .cloned() converts &T references back to owned T values
}

// Recursive: mirrors OCaml's pattern match
pub fn filter_rec<T: Clone, P: Fn(&T) -> bool>(list: &[T], p: P) -> Vec<T> {
    fn go<T: Clone>(list: &[T], p: &dyn Fn(&T) -> bool) -> Vec<T> {
        match list {
            [] => vec![],
            [head, tail @ ..] => {
                let rest = go(tail, p);
                if p(head) {
                    // prepend head to rest — preserves order
                    let mut result = vec![head.clone()];
                    result.extend(rest);
                    result
                } else {
                    rest  // skip head
                }
            }
        }
    }
    go(list, &p)
    // Note: &dyn Fn breaks the infinite type recursion that &p would cause
}

// Fold: filter as fold
pub fn filter_fold<T: Clone, P: Fn(&T) -> bool>(list: &[T], p: P) -> Vec<T> {
    list.iter().fold(Vec::new(), |mut acc, x| {
        if p(x) { acc.push(x.clone()); }
        acc
    })
}
```

The `&dyn Fn` in the recursive version breaks an otherwise infinite type: passing `&p` in a generic recursive function would create `&&p`, `&&&p`, … infinitely. Dynamic dispatch via `&dyn Fn` stops the monomorphization chain.

## What This Unlocks

- **Selection from collections** — keep only the elements that matter for the current task.
- **Understanding `dyn Fn`** — the recursive version teaches *why* Rust sometimes needs `&dyn Fn` in recursive generic code.
- **Fold's universality confirmed** — `filter_fold` shows that filter is a fold, just like `map`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Predicate type | `'a -> bool` | `Fn(&T) -> bool` |
| Partial application | `let evens = filter (fun n -> n mod 2 = 0)` | Closure: `\|nums\| filter(nums, \|n\| n % 2 == 0)` |
| Recursive generic | Polymorphic recursion straightforward | Needs `&dyn Fn` break-point |
| Order guarantee | Left-to-right, guaranteed | Same — both preserve relative order |
| `cloned()` | Not needed (GC copies) | Required to get `T` from `&T` iterator |
