# OCaml vs Rust: Default Trait

## Side-by-Side Code

### OCaml — Record default via let binding
```ocaml
type config = {
  host: string;
  port: int;
  debug: bool;
}

let default_config = {
  host = "localhost";
  port = 8080;
  debug = false;
}

(* Struct update *)
let custom = { default_config with port = 9000 }
```

### Rust — Default trait
```rust
#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
        }
    }
}

// Struct update syntax
let custom = Config {
    port: 9000,
    ..Default::default()
};
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Default values | Named let binding | `Default` trait |
| Derivable | No | `#[derive(Default)]` for zero-values |
| Struct update | `{ record with field = value }` | `{ field, ..Default::default() }` |
| Collection default | Not standardized | `or_default()`, `unwrap_or_default()` |
| Generic bound | N/A | `T: Default` |

---

## Derive vs Custom

```rust
// Derive: all fields use their Default (0, false, "", etc.)
#[derive(Default)]
struct Point { x: i32, y: i32 }
// Point::default() → Point { x: 0, y: 0 }

// Custom: you choose the values
impl Default for Config {
    fn default() -> Self {
        Config { host: "localhost".into(), port: 8080, debug: false }
    }
}
```

---

## Common Patterns

### or_default() in Collections
```rust
let mut counts: HashMap<&str, u32> = HashMap::new();
*counts.entry("key").or_default() += 1;  // u32::default() = 0
```

### unwrap_or_default()
```rust
let opt: Option<Vec<i32>> = None;
let v = opt.unwrap_or_default();  // Vec::default() = []
```

### Generic Functions
```rust
fn ensure<T: Default>(opt: Option<T>) -> T {
    opt.unwrap_or_default()
}
```

---

## 5 Takeaways

1. **`#[derive(Default)]` works for zero-like defaults.**
   Numbers → 0, bools → false, strings → empty.

2. **Custom `impl Default` for meaningful defaults.**
   Port 8080, timeout 30s, etc.

3. **Struct update syntax uses `..Default::default()`.**
   Override specific fields, fill rest with defaults.

4. **`or_default()` is idiomatic for HashMap counters.**
   No need for `entry().or_insert(0)`.

5. **OCaml uses named values; Rust uses a trait.**
   `default_config` vs `Config::default()`.
