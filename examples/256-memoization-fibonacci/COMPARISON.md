# OCaml vs Rust: Memoization — Fibonacci with Hashtable Cache

## Side-by-Side Code

### OCaml
```ocaml
let memoize f =
  let cache = Hashtbl.create 16 in
  fun x ->
    match Hashtbl.find_opt cache x with
    | Some v -> v
    | None ->
      let v = f x in
      Hashtbl.add cache x v;
      v

let fib =
  let rec fib' n =
    if n <= 1 then n
    else memo_fib (n - 1) + memo_fib (n - 2)
  and memo_fib = memoize fib'
  in memo_fib
```

### Rust (idiomatic — struct-based)
```rust
use std::collections::HashMap;

pub struct FibMemo {
    cache: HashMap<u64, u64>,
}

impl FibMemo {
    pub fn new() -> Self { Self { cache: HashMap::new() } }

    pub fn fib(&mut self, n: u64) -> u64 {
        if let Some(&v) = self.cache.get(&n) { return v; }
        let v = if n <= 1 { n } else { self.fib(n - 1) + self.fib(n - 2) };
        self.cache.insert(n, v);
        v
    }
}
```

### Rust (generic HOF memoize — mirrors OCaml)
```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

pub fn memoize<A, R, F>(f: F) -> impl FnMut(A) -> R
where
    A: Eq + Hash + Clone,
    R: Clone,
    F: Fn(A) -> R,
{
    let cache = RefCell::new(HashMap::new());
    move |x: A| {
        if let Some(v) = cache.borrow().get(&x).cloned() { return v; }
        let v = f(x.clone());
        cache.borrow_mut().insert(x, v.clone());
        v
    }
}

// Recursive memoization requires threading the cache explicitly
pub fn fib_hof(n: u64) -> u64 {
    let cache = RefCell::new(HashMap::<u64, u64>::new());
    fn inner(n: u64, cache: &RefCell<HashMap<u64, u64>>) -> u64 {
        if let Some(&v) = cache.borrow().get(&n) { return v; }
        let v = if n <= 1 { n } else { inner(n-1, cache) + inner(n-2, cache) };
        cache.borrow_mut().insert(n, v);
        v
    }
    inner(n, &cache)
}
```

### Rust (thread-local transparent memoization)
```rust
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static FIB_CACHE: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
}

pub fn fib_tl(n: u64) -> u64 {
    if let Some(v) = FIB_CACHE.with(|c| c.borrow().get(&n).copied()) {
        return v;
    }
    let v = if n <= 1 { n } else { fib_tl(n - 1) + fib_tl(n - 2) };
    FIB_CACHE.with(|c| c.borrow_mut().insert(n, v));
    v
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Generic memoize | `val memoize : ('a -> 'b) -> 'a -> 'b` | `fn memoize<A,R,F>(f: F) -> impl FnMut(A) -> R` |
| Cache type | `('a, 'b) Hashtbl.t` | `HashMap<A, R>` |
| Interior mutability | implicit (GC + mutation) | `RefCell<HashMap<…>>` |
| Transparent global cache | module-level `let cache = Hashtbl…` | `thread_local! { static … }` |
| Mutable self reference | `let rec … and` | `&mut self` method or explicit parameter |

## Key Insights

1. **No `let rec … and` for closures:** OCaml's mutual recursion at the binding level lets `fib'` and `memo_fib` reference each other as if simultaneously defined. Rust closures cannot do this — the workarounds are explicit cache parameters (HOF style) or module-level state (`thread_local!`).

2. **`RefCell` is runtime borrow checking:** OCaml's GC allows the closure to mutate the `Hashtbl` freely. Rust requires `RefCell<HashMap>` to defer the borrow check to runtime, enabling mutation inside an `Fn` closure that is otherwise captured by immutable reference.

3. **Mutable state is visible in Rust types:** `FibMemo::fib(&mut self)` advertises in its signature that the cache is modified. OCaml hides this behind the closure — convenient but less traceable in large codebases.

4. **`thread_local!` scope vs OCaml's module scope:** OCaml's module-level `Hashtbl` is shared within a process (single-threaded assumption). Rust's `thread_local!` gives each thread its own copy — safer by default, but requires `Mutex<HashMap>` for cross-thread sharing.

5. **The generic `memoize` HOF is valid but limited:** Both OCaml and Rust can express a generic memoize wrapper. The limitation in Rust is that the wrapped `f` receives no reference back to the memoized version of itself, so naively applying `memoize` to a recursive function memoizes only the top-level call.

## When to Use Each Style

**Use struct-based (`FibMemo`) when:** you want to carry the cache across multiple calls, share it explicitly, or need to inspect/clear it. This is the most idiomatic Rust pattern.

**Use the HOF `memoize` wrapper when:** you are wrapping non-recursive functions (query deduplication, expensive pure computations) and want a composable, reusable abstraction.

**Use `thread_local!` when:** you want call-site transparency — callers invoke `fib_tl(n)` with no extra arguments, matching the OCaml experience. Suitable when the cache should persist across calls within a thread.
