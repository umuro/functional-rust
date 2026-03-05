# OCaml vs Rust: Test Fixtures

## RAII Teardown Pattern

### OCaml
```ocaml
let with_database name f =
  let db = setup_database name in
  Fun.protect ~finally:(fun () -> teardown_database db) (fun () -> f db)

let%test "lookup existing" =
  with_database "test" (fun db ->
    Db.get db "user:1" = Some "Alice"
  )
```

### Rust
```rust
struct DatabaseFixture {
    pub db: Database,
    name: &'static str,
}

impl DatabaseFixture {
    fn new(name: &'static str) -> Self {
        let db = Database::with_test_data();
        DatabaseFixture { db, name }
    }
}

impl Drop for DatabaseFixture {
    fn drop(&mut self) {
        // Teardown runs even if test panics!
    }
}

#[test]
fn test_lookup_existing() {
    let f = DatabaseFixture::new("lookup");
    assert_eq!(f.db.get("user:1"), Some("Alice"));
}
```

## Shared Read-Only State

### OCaml
```ocaml
let shared_data = lazy (List.init 100 (fun i -> i + 1))

let%test "sum is 5050" =
  let data = Lazy.force shared_data in
  List.fold_left (+) 0 data = 5050
```

### Rust
```rust
static SHARED_DATA: OnceLock<Vec<i32>> = OnceLock::new();

fn shared_data() -> &'static [i32] {
    SHARED_DATA.get_or_init(|| (1..=100).collect())
}

#[test]
fn test_sum() {
    let sum: i32 = shared_data().iter().sum();
    assert_eq!(sum, 5050);
}
```

## Builder Pattern for Fixtures

### Rust
```rust
let db = DatabaseBuilder::new()
    .with_user(1, "Alice")
    .with_user(2, "Bob")
    .with_entry("config:timeout", "30")
    .build();
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Teardown | `Fun.protect ~finally:` | `Drop` trait |
| Panic safety | Exception-safe with `protect` | Drop runs even on panic |
| Lazy init | `lazy` keyword | `OnceLock::get_or_init` |
| Thread safety | Not by default | `OnceLock` is thread-safe |
| Builder pattern | Optional args / record update | Method chaining |
