#![allow(clippy::all)]
//! Lazy Static Pattern
//!
//! Lazy initialization of static values.

use std::sync::OnceLock;

/// Global config using OnceLock.
static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub debug: bool,
    pub max_size: usize,
}

impl Config {
    pub fn global() -> &'static Config {
        CONFIG.get_or_init(|| Config {
            debug: cfg!(debug_assertions),
            max_size: 1024,
        })
    }
}

/// Thread-local state.
thread_local! {
    static COUNTER: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

pub fn increment_counter() -> u32 {
    COUNTER.with(|c| {
        let v = c.get() + 1;
        c.set(v);
        v
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_global() {
        let cfg = Config::global();
        assert_eq!(cfg.max_size, 1024);
    }

    #[test]
    fn test_config_same_instance() {
        let cfg1 = Config::global();
        let cfg2 = Config::global();
        assert!(std::ptr::eq(cfg1, cfg2));
    }

    #[test]
    fn test_thread_local_counter() {
        let v1 = increment_counter();
        let v2 = increment_counter();
        assert_eq!(v2, v1 + 1);
    }

    #[test]
    fn test_config_debug() {
        let cfg = Config::global();
        // In tests, debug_assertions is typically true
        #[cfg(debug_assertions)]
        assert!(cfg.debug);
    }

    #[test]
    fn test_multiple_increments() {
        let start = increment_counter();
        increment_counter();
        increment_counter();
        let end = increment_counter();
        assert_eq!(end, start + 3);
    }
}
