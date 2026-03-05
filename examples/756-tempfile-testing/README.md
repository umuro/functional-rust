📖 **[View on hightechmind.io →](https://hightechmind.io/rust/756-tempfile-testing)**

---

# 756: Testing with Temporary Files and Directories

**Difficulty:** 2  **Level:** Intermediate

RAII temporary directories for filesystem tests — automatic cleanup even on panic, with per-test isolation.

## The Problem This Solves

Functions that read from or write to disk require a real filesystem to test properly. But tests shouldn't leave debris: leftover files in `/tmp` accumulate, pollute other test runs, and can cause flaky failures when a test assumes a clean directory. The solution is a `TempDir` struct: it creates a unique directory in `new()`, your test uses it, and `Drop` removes it when the test ends — even if the test panics.

This is the RAII pattern applied to filesystem resources. The same approach powers the `tempfile` crate (the standard library for this in production Rust), but understanding the hand-rolled version makes the pattern clear. It's also more portable across environments where you can't add dependencies.

Every test gets its own unique directory (using `AtomicU64` + process ID), so parallel test execution (`cargo test -- --test-threads 4`) never has contention. The test writes into its own isolated space, and cleanup is automatic.

## The Intuition

A `TempDir` struct holds a `PathBuf`. `new()` creates a directory at a unique path under `std::env::temp_dir()`. `path()` and `child()` return paths inside it. `Drop::drop()` calls `fs::remove_dir_all()` — the equivalent of `rm -rf`. Since `Drop` runs on scope exit *and* on panic, you get guaranteed cleanup with no try/finally needed.

## How It Works in Rust

```rust
static COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct TempDir { path: PathBuf }

impl TempDir {
    pub fn new() -> io::Result<Self> {
        // Unique name: combines process ID + atomic counter
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let name = format!("rust_test_{}_{}", std::process::id(), id);
        let path = std::env::temp_dir().join(name);
        fs::create_dir_all(&path)?;
        Ok(TempDir { path })
    }

    pub fn path(&self) -> &Path { &self.path }
    pub fn child(&self, name: &str) -> PathBuf { self.path.join(name) }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = fs::remove_dir_all(&self.path);  // ignore errors in Drop
        }
    }
}

// Using it in a test
#[test]
fn test_write_and_read() {
    let dir = TempDir::new().unwrap();        // creates /tmp/rust_test_12345_0/
    let file = dir.child("data.txt");

    write_lines(&file, &["line1", "line2"]).unwrap();
    assert_eq!(count_lines(&file).unwrap(), 2);
    // dir drops here → /tmp/rust_test_12345_0/ deleted automatically
}

#[test]
fn cleanup_on_drop() {
    let path = {
        let dir = TempDir::new().unwrap();
        let p = dir.path().to_path_buf();
        assert!(p.exists());
        p   // dir dropped here
    };
    assert!(!path.exists()); // directory is gone
}
```

`let _ = fs::remove_dir_all(...)` in `Drop` ignores errors — it's considered bad practice to panic in `Drop`, since panicking during a panic causes abort.

## What This Unlocks

- **Filesystem tests without side effects** — any function that reads/writes files can be tested cleanly without manual setup/teardown; tests are hermetic and can run in parallel.
- **`Drop` as the universal cleanup hook** — this pattern extends beyond files: database connections, lock files, test servers, and any scoped resource that needs release on scope exit.
- **Unique paths via `AtomicU64`** — combining process ID with an atomic counter is the standard technique for generating unique names in multi-process, multi-threaded environments without coordination.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Temp directory | `Filename.temp_dir` + manual cleanup | `TempDir` struct with `Drop` — automatic |
| Unique ID generation | `Unix.getpid ()` + counter | `std::process::id()` + `AtomicU64` |
| Panic-safe cleanup | `Fun.protect ~finally:` | `Drop` — always runs, no special syntax |
| Ignore errors in Drop | `try _ with _ -> ()` | `let _ = expr;` — explicit discard |
