📖 **[View on hightechmind.io →](https://hightechmind.io/rust/528-closure-lifetime-capture)**

---

# 528: Closures Capturing References

**Difficulty:** 4  **Level:** Intermediate-Advanced

When closures borrow instead of own, their lifetimes are constrained — understanding this explains the most confusing closure errors.

## The Problem This Solves

You write `let checker = make_prefix_checker(&prefix)` and then try to store `checker` in a struct or return it — only to get lifetime errors: `checker does not live long enough` or `borrowed value must be valid for the lifetime 'a`. You add lifetime annotations and it still doesn't work.

The root cause: a closure that captures `&T` borrows the owner for the closure's entire lifetime. The closure *is* the borrow. You can't use the original variable while the closure exists (for mutable captures), and you can't let the closure outlive the variable.

These errors protect you from use-after-free bugs. But without understanding the mechanics, they feel arbitrary and the solutions seem like guesswork.

## The Intuition

A closure capturing a reference is like a sticky note attached to a book. As long as the sticky note exists, the book must exist too. You can't throw out the book while the note is still attached to it.

In Python and JavaScript, the GC handles this automatically — the closure holds a reference that keeps the object alive. Rust's borrow checker does the same job at compile time, without GC overhead: if the closure exists, the borrowed data must also exist.

The fix depends on what you need:
- **Keep using both**: structure your code so the closure is dropped before the owner.
- **Return the closure**: use `impl Fn(...) + 'a` to thread the lifetime through.
- **Closure must outlive scope**: use `move` to transfer ownership instead of borrowing.

## How It Works in Rust

```rust
// Closure captures &str — its lifetime is tied to prefix
fn make_prefix_checker<'a>(prefix: &'a str) -> impl Fn(&str) -> bool + 'a {
    //                 ^^                                                 ^^
    //   lifetime of the borrow                    closure inherits this lifetime
    move |s| s.starts_with(prefix)   // prefix is a &str reference — owned by string
}

let prefix = String::from("hello");
let checker = make_prefix_checker(&prefix);   // checker borrows prefix
println!("{}", checker("hello world"));        // ✓

// Correct order: drop checker BEFORE dropping prefix
drop(checker);   // borrow released
drop(prefix);    // now safe to drop

// Struct holding a closure that borrows — struct needs lifetime parameter
struct Filter<'a, T> {
    data: &'a [T],
    predicate: Box<dyn Fn(&T) -> bool + 'a>,   // 'a: predicate can't outlive data
}
impl<'a, T> Filter<'a, T> {
    fn new(data: &'a [T], pred: impl Fn(&T) -> bool + 'a) -> Self {
        Filter { data, predicate: Box::new(pred) }
    }
    fn apply(&self) -> Vec<&T> {
        self.data.iter().filter(|x| (self.predicate)(x)).collect()
    }
}

let numbers = vec![1, 2, 3, 4, 5, 6];
let threshold = 3;
// threshold is captured by reference — Filter lifetime tied to threshold
let filter = Filter::new(&numbers, move |&x| x > threshold);
println!("{:?}", filter.apply()); // [4, 5, 6]

// Multiple shared borrows are fine — closures only read
let data = vec![1, 2, 3, 4, 5];
let sum_closure = || data.iter().sum::<i32>();   // shared borrow of data
let max_closure = || data.iter().max().copied(); // another shared borrow — fine
println!("{} {:?}", sum_closure(), max_closure()); // both can coexist

// This WOULD NOT compile:
// let checker2;
// { let local = String::from("temp");
//   checker2 = make_prefix_checker(&local); }  // local dropped here
// checker2("test"); // ✗ use after free — caught at compile time
```

## What This Unlocks

- **Zero-copy predicates** — pass `&str` and `&[T]` references into closures for filtering/searching without cloning.
- **Borrow-checker literacy** — understanding captured-reference lifetimes explains `closure may outlive the current function` and `borrowed value does not live long enough`.
- **Lifetime annotations on structs** — any struct holding a closure that borrows needs a lifetime parameter; this pattern is the gateway to understanding that requirement.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Reference capture | Implicit — GC keeps referent alive | Inferred lifetime borrow — must not outlive referent |
| Closure lifetime | GC handles — can store anywhere | Cannot outlive the borrowed values |
| Return borrowing closure | No issue — GC | Requires explicit `+ 'a` lifetime annotation |
| Mutable reference capture | Via `ref` | Exclusive borrow — no other access while closure exists |
| Fix: outlive scope | N/A | Use `move` to transfer ownership instead |
