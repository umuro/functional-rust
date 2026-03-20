[once-init on hightechmind.io](https://hightechmind.io/posts/functional-rust/once-init)

---

## Problem Statement

Demonstrate one-time initialization using `OnceLock<T>` — a global that is set exactly once and then read-only for the lifetime of the program. Multiple concurrent threads may call `get_or_init` simultaneously; only one initialization runs. Use for global configuration, pre-computed prime sieves, and expensive singleton setup.

## Learning Outcomes

- Declare `static CONFIG: OnceLock<String>` and initialize with `CONFIG.get_or_init(|| ...)`
- Understand that `get_or_init` guarantees the initializer runs exactly once, even under concurrent calls
- Use `OnceLock<Vec<u32>>` for expensive pre-computed data (prime sieve) initialized on first use
- Recognize `OnceLock<T>` vs `Lazy<T>` (from `once_cell`/`std::sync::LazyLock` in Rust 1.80+)
- Connect to OCaml's `lazy` keyword and `Lazy.force`

## Rust Application

```rust
static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static String {
    CONFIG.get_or_init(|| {
        // Only runs once — even with concurrent callers
        "production-config-v42".to_string()
    })
}

static PRIMES: OnceLock<Vec<u32>> = OnceLock::new();

fn get_primes() -> &'static [u32] {
    PRIMES.get_or_init(|| sieve(10_000))
}

fn sieve(limit: usize) -> Vec<u32> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit > 0 { is_prime[1] = false; }
    for i in 2..=limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit { is_prime[j] = false; j += i; }
        }
    }
    (2..=limit as u32).filter(|&n| is_prime[n as usize]).collect()
}
```

`OnceLock<T>` holds an `Option<T>` internally plus a `Once` synchronization primitive. `get_or_init` acquires an internal lock, checks if initialized, runs the initializer if not, then releases the lock. All subsequent calls are lock-free reads.

The returned `&'static String` or `&'static [u32]` is safe because the value lives for the entire program lifetime (stored in a `static`).

## OCaml Approach

```ocaml
(* OCaml lazy values — initialized on first force *)
let config = lazy "production-config-v42"
let get_config () = Lazy.force config

(* Lazy computation *)
let primes = lazy (sieve 10_000)
let get_primes () = Lazy.force primes

(* Thread-safe lazy (OCaml 5.0+) *)
(* Lazy.force is thread-safe: only one thread runs the initializer *)
let expensive = lazy begin
  Printf.printf "initializing...\n%!";
  heavy_computation ()
end
```

OCaml's `lazy expr` creates a suspended computation that runs once on `Lazy.force`. In OCaml 5.0+, `Lazy.force` is thread-safe — concurrent forces compete to run the computation, and only one wins. This is identical to `OnceLock::get_or_init`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| One-time init | `OnceLock::get_or_init` | `lazy` + `Lazy.force` |
| Return type | `&'static T` — guaranteed static lifetime | `'a Lazy.t` — suspended value |
| Thread safety | Guaranteed by `OnceLock` | Guaranteed by `Lazy.force` (OCaml 5+) |
| Lazy evaluation | `OnceLock` + closure (explicit) | `lazy` keyword (syntactic) |
| Rust 1.80+ | `std::sync::LazyLock<T>` | Already has `lazy` |

`OnceLock` is preferred over `std::sync::Once` when the initialized value needs to be returned. `LazyLock<T>` (stable in Rust 1.80+) provides the `lazy`-like pattern: `static X: LazyLock<T> = LazyLock::new(|| expensive_init())`.

## Exercises

1. Use `LazyLock` (Rust 1.80+) instead of `OnceLock` to remove the explicit `get_or_init` call.
2. Implement a `once_cell`-style `OnceCell<T>` manually using `UnsafeCell<Option<T>>` + `Once`.
3. Initialize the prime sieve on first use and benchmark first vs second call to verify one-time execution.
4. Implement a global `RequestCounter` that is initialized to 0 and incremented atomically — combine `OnceLock` with `AtomicUsize`.
5. Implement a `per_thread_once`: each thread has its own one-time-initialized value using `thread_local! { static: OnceLock<T> }`.
