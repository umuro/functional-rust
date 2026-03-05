/// 758: Test Isolation — avoiding shared mutable state
/// cargo test runs tests in parallel — global state = flaky tests!

use std::sync::{Mutex, OnceLock, atomic::{AtomicU32, Ordering}};

// ── Code under test ────────────────────────────────────────────────────────────

pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn new(initial: u32) -> Self { Counter { value: initial } }
    pub fn increment(&mut self) { self.value += 1; }
    pub fn add(&mut self, n: u32) { self.value += n; }
    pub fn value(&self) -> u32 { self.value }
    pub fn reset(&mut self) { self.value = 0; }
}

pub struct Registry {
    entries: Vec<String>,
}

impl Registry {
    pub fn new() -> Self { Registry { entries: Vec::new() } }
    pub fn register(&mut self, name: &str) { self.entries.push(name.to_owned()); }
    pub fn contains(&self, name: &str) -> bool { self.entries.iter().any(|e| e == name) }
    pub fn count(&self) -> usize { self.entries.len() }
}

fn main() {
    let mut c = Counter::new(0);
    c.increment();
    c.add(5);
    println!("Counter: {}", c.value());
}

// ── Test patterns ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── ANTI-PATTERN: global mutable static (don't do this) ──────────────────
    //
    // static GLOBAL_COUNTER: AtomicU32 = AtomicU32::new(0);
    //
    // #[test]
    // fn bad_test_1() {
    //     GLOBAL_COUNTER.store(0, Ordering::SeqCst);  // must reset — fragile!
    //     GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
    //     assert_eq!(GLOBAL_COUNTER.load(Ordering::SeqCst), 1);
    //     // Race: another test might read/write GLOBAL_COUNTER concurrently!
    // }

    // ── PATTERN 1: Per-test instance — best practice ──────────────────────────

    #[test]
    fn counter_increments_from_zero() {
        let mut c = Counter::new(0);  // fresh instance, no sharing
        c.increment();
        assert_eq!(c.value(), 1);
    }

    #[test]
    fn counter_add_five() {
        let mut c = Counter::new(0);  // fresh instance, independent
        c.add(5);
        assert_eq!(c.value(), 5);
    }

    #[test]
    fn counter_reset() {
        let mut c = Counter::new(100);
        c.reset();
        assert_eq!(c.value(), 0);
    }

    // ── PATTERN 2: Read-only shared state via OnceLock ────────────────────────

    static SHARED_DATA: OnceLock<Vec<u32>> = OnceLock::new();

    fn shared_data() -> &'static [u32] {
        SHARED_DATA.get_or_init(|| {
            // This runs exactly once, even with concurrent tests
            (0u32..=100).map(|i| i * i).collect()
        })
    }

    #[test]
    fn shared_data_index_3() {
        assert_eq!(shared_data()[3], 9);   // 3² = 9
    }

    #[test]
    fn shared_data_index_10() {
        assert_eq!(shared_data()[10], 100);  // 10² = 100
    }

    #[test]
    fn shared_data_length() {
        assert_eq!(shared_data().len(), 101);  // 0..=100
    }

    // ── PATTERN 3: Mutex-guarded shared mutable state (when necessary) ────────

    static GLOBAL_REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();

    fn registry() -> &'static Mutex<Registry> {
        GLOBAL_REGISTRY.get_or_init(|| Mutex::new(Registry::new()))
    }

    #[test]
    fn registry_register_and_contains() {
        // Use a unique name to avoid test-order dependency
        let unique = format!("service_registry_test_{}", std::thread::current().id().as_u64().get());
        registry().lock().unwrap().register(&unique);
        assert!(registry().lock().unwrap().contains(&unique));
    }

    // ── PATTERN 4: Environment variable isolation ──────────────────────────────
    // env::set_var() is global — use dedicated env-wrapper or avoid in parallel tests.

    struct EnvGuard {
        key:      &'static str,
        original: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let original = std::env::var(key).ok();
            // SAFETY: only safe in single-threaded tests or with external serialization
            unsafe { std::env::set_var(key, value); }
            EnvGuard { key, original }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.original {
                Some(v) => unsafe { std::env::set_var(self.key, v); },
                None    => unsafe { std::env::remove_var(self.key); },
            }
        }
    }

    // NOTE: This test can't run in parallel with other env-touching tests.
    // In a real project, serialize env-mutating tests with a Mutex.
    #[test]
    fn env_guard_restores_on_drop() {
        let key = "MY_TEST_UNIQUE_VAR_758";
        std::env::remove_var(key);  // ensure clean state
        assert!(std::env::var(key).is_err());
        {
            let _guard = EnvGuard::set(key, "test_value");
            assert_eq!(std::env::var(key).unwrap(), "test_value");
        }   // guard dropped here — restores env
        assert!(std::env::var(key).is_err());
    }

    // ── PATTERN 5: Per-test registry (pure isolation) ─────────────────────────

    #[test]
    fn registry_per_test_instance_is_isolated() {
        let mut r1 = Registry::new();
        let mut r2 = Registry::new();   // independent
        r1.register("alice");
        r2.register("bob");
        assert!(r1.contains("alice"));
        assert!(!r1.contains("bob"),   "r1 should not see r2's entries");
        assert!(r2.contains("bob"));
        assert!(!r2.contains("alice"), "r2 should not see r1's entries");
    }
}
