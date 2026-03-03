use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

// ── Solution 1: Struct-based memoization ─────────────────────────────────────

pub struct FibMemo {
    cache: HashMap<u64, u64>,
}

impl Default for FibMemo {
    fn default() -> Self {
        Self::new()
    }
}

impl FibMemo {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn fib(&mut self, n: u64) -> u64 {
        if let Some(&v) = self.cache.get(&n) {
            return v;
        }
        let v = if n <= 1 {
            n
        } else {
            self.fib(n - 1) + self.fib(n - 2)
        };
        self.cache.insert(n, v);
        v
    }
}

pub fn fib_struct(n: u64) -> u64 {
    FibMemo::new().fib(n)
}

// ── Solution 2: Generic memoize HOF + explicit-cache recursion ───────────────

pub fn memoize<A, R, F>(f: F) -> impl FnMut(A) -> R
where
    A: Eq + Hash + Clone,
    R: Clone,
    F: Fn(A) -> R,
{
    let cache = RefCell::new(HashMap::new());
    move |x: A| {
        if let Some(v) = cache.borrow().get(&x).cloned() {
            return v;
        }
        let v = f(x.clone());
        cache.borrow_mut().insert(x, v.clone());
        v
    }
}

pub fn fib_hof(n: u64) -> u64 {
    let cache = RefCell::new(HashMap::<u64, u64>::new());

    fn inner(n: u64, cache: &RefCell<HashMap<u64, u64>>) -> u64 {
        if let Some(&v) = cache.borrow().get(&n) {
            return v;
        }
        let v = if n <= 1 {
            n
        } else {
            inner(n - 1, cache) + inner(n - 2, cache)
        };
        cache.borrow_mut().insert(n, v);
        v
    }

    inner(n, &cache)
}

// ── Solution 3: Thread-local transparent memoization ─────────────────────────

thread_local! {
    static FIB_CACHE: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
}

pub fn fib_tl(n: u64) -> u64 {
    if let Some(v) = FIB_CACHE.with(|c| c.borrow().get(&n).copied()) {
        return v;
    }
    let v = if n <= 1 {
        n
    } else {
        fib_tl(n - 1) + fib_tl(n - 2)
    };
    FIB_CACHE.with(|c| c.borrow_mut().insert(n, v));
    v
}

fn main() {
    // ── struct-based ──────────────────────────────────────────────────────────
    let mut m = FibMemo::new();
    println!("=== Struct-based memoization ===");
    println!("fib(0)  = {}", m.fib(0));
    println!("fib(1)  = {}", m.fib(1));
    println!("fib(10) = {}", m.fib(10));
    println!("fib(35) = {}", m.fib(35));

    // ── generic memoize HOF ───────────────────────────────────────────────────
    println!("\n=== Generic memoize (HOF) ===");
    let mut sq = memoize(|x: u64| x * x);
    println!("sq(7)   = {} (first call)", sq(7));
    println!("sq(7)   = {} (cache hit)", sq(7));
    println!("fib_hof(35) = {}", fib_hof(35));

    // ── thread-local ──────────────────────────────────────────────────────────
    println!("\n=== Thread-local transparent memoization ===");
    println!("fib_tl(35) = {}", fib_tl(35));
}

/* Output:
   === Struct-based memoization ===
   fib(0)  = 0
   fib(1)  = 1
   fib(10) = 55
   fib(35) = 9227465

   === Generic memoize (HOF) ===
   sq(7)   = 49 (first call)
   sq(7)   = 49 (cache hit)
   fib_hof(35) = 9227465

   === Thread-local transparent memoization ===
   fib_tl(35) = 9227465
*/
