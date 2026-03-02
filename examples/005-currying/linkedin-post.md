# LinkedIn Post: Currying and Partial Application

🦀 **Functional Rust #005: Currying and Partial Application**

OCaml curries automatically. Rust requires closures. Here's why.

**OCaml (built-in):**
```ocaml
let add x y = x + y  (* Already int -> int -> int *)
let add5 = add 5     (* Partial application *)
```

**Rust (manual):**
```rust
fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
let add5 = add(5);
```

**Why Rust doesn't auto-curry:**

⚡ **Performance** - Avoid closure overhead
🔒 **Ownership** - Capturing variables is explicit
🎯 **Ergonomics** - Method chaining > nested calls

**Rust's answer: closures**
```rust
let add5 = |y| 5 + y;
vec![1, 2, 3].iter().map(add5).collect()
```

Currying is elegant in OCaml. In Rust, closures + iterators achieve the same composability with better performance.

Next: Function composition 🔗

#Rust #FunctionalProgramming #Closures #RustLang #Performance
