#![allow(clippy::all)]
// 456. OnceLock and OnceCell for lazy init
use std::cell::OnceCell;
use std::collections::HashMap;
use std::sync::OnceLock;

static CONFIG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
static GREETING: OnceLock<String> = OnceLock::new();

fn config() -> &'static HashMap<&'static str, &'static str> {
    CONFIG.get_or_init(|| {
        println!("init config");
        [("host", "localhost"), ("port", "8080")]
            .iter()
            .cloned()
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::OnceLock;
    #[test]
    fn test_once_only() {
        let lock: OnceLock<u32> = OnceLock::new();
        let n = AtomicU32::new(0);
        lock.get_or_init(|| {
            n.fetch_add(1, Ordering::SeqCst);
            42
        });
        lock.get_or_init(|| {
            n.fetch_add(1, Ordering::SeqCst);
            99
        });
        assert_eq!(*lock.get().unwrap(), 42);
        assert_eq!(n.load(Ordering::SeqCst), 1);
    }
}
