# LinkedIn Post: List Operations and Recursion

🦀 **Functional Rust #002: List Operations and Recursion**

OCaml uses linked lists with cons cells. Rust uses slices and vectors. How do recursive list operations translate?

**OCaml:**
```ocaml
let rec sum lst =
  match lst with
  | [] -> 0
  | head :: tail -> head + sum tail
```

**Rust:**
```rust
fn sum(lst: &[i32]) -> i32 {
    match lst {
        [] => 0,
        [head, tail @ ..] => head + sum(tail),
    }
}
```

**Key differences:**

📌 **Data structures** - Linked lists vs contiguous arrays
📌 **Pattern syntax** - `head :: tail` vs `[head, tail @ ..]`
📌 **Memory** - GC vs ownership (requires explicit cloning)
📌 **Performance** - O(n) access vs O(1) indexing

**Tail recursion matters in both:**

OCaml guarantees tail-call optimization. Rust doesn't, but LLVM usually delivers in release builds.

```rust
fn sum_tr(lst: &[i32]) -> i32 {
    fn aux(acc: i32, lst: &[i32]) -> i32 {
        match lst {
            [] => acc,
            [head, tail @ ..] => aux(acc + head, tail),
        }
    }
    aux(0, lst)
}
```

**In practice:** Rust iterators are more idiomatic than recursion for list operations. But understanding the recursive patterns helps you think functionally in a systems language.

Next up: Pattern matching deep dive 🔍

#Rust #FunctionalProgramming #OCaml #SystemsProgramming #RustLang
