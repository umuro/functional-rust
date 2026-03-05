//! Complex Closure Environments
//!
//! Closures capturing structs, collections, and other closures.

/// Configuration for a formatter.
pub struct Config {
    pub prefix: String,
    pub max_len: usize,
    pub transform: Box<dyn Fn(String) -> String>,
}

/// Closure capturing a Config struct.
pub fn make_formatter(cfg: Config) -> impl FnMut(&str) -> String {
    move |s: &str| {
        let truncated = if s.len() > cfg.max_len {
            format!("{}...", &s[..cfg.max_len])
        } else {
            s.to_string()
        };
        (cfg.transform)(format!("{}{}", cfg.prefix, truncated))
    }
}

/// Closure capturing a Vec and an index — cyclic iterator.
pub fn make_cycler<T: Clone>(items: Vec<T>) -> impl FnMut() -> T {
    let mut index = 0;
    move || {
        let val = items[index].clone();
        index = (index + 1) % items.len();
        val
    }
}

/// Closure capturing another closure.
pub fn make_logged_fn<A, B, F>(f: F, name: &str) -> impl Fn(A) -> B
where
    F: Fn(A) -> B,
{
    let name = name.to_string();
    move |a| {
        // In real code, this would log
        let _ = &name; // use the captured name
        f(a)
    }
}

/// Counter that captures mutable state.
pub fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        let current = count;
        count += 1;
        current
    }
}

/// Accumulator that captures a Vec.
pub fn make_accumulator<T: Clone>() -> impl FnMut(T) -> Vec<T> {
    let mut items: Vec<T> = Vec::new();
    move |item: T| {
        items.push(item);
        items.clone()
    }
}

/// Closure capturing a HashMap.
pub fn make_cache<K, V, F>(compute: F) -> impl FnMut(K) -> V
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
    F: Fn(&K) -> V,
{
    let mut cache = std::collections::HashMap::new();
    move |key: K| {
        cache
            .entry(key.clone())
            .or_insert_with(|| compute(&key))
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_formatter() {
        let cfg = Config {
            prefix: "[INFO] ".to_string(),
            max_len: 10,
            transform: Box::new(|s| s.to_uppercase()),
        };
        let mut fmt = make_formatter(cfg);

        assert_eq!(fmt("hello"), "[INFO] HELLO");
        assert_eq!(fmt("this is a very long message"), "[INFO] THIS IS A ...");
    }

    #[test]
    fn test_make_cycler() {
        let mut cycler = make_cycler(vec!["a", "b", "c"]);
        assert_eq!(cycler(), "a");
        assert_eq!(cycler(), "b");
        assert_eq!(cycler(), "c");
        assert_eq!(cycler(), "a"); // wraps around
    }

    #[test]
    fn test_make_counter() {
        let mut counter = make_counter(10);
        assert_eq!(counter(), 10);
        assert_eq!(counter(), 11);
        assert_eq!(counter(), 12);
    }

    #[test]
    fn test_make_accumulator() {
        let mut acc = make_accumulator();
        assert_eq!(acc(1), vec![1]);
        assert_eq!(acc(2), vec![1, 2]);
        assert_eq!(acc(3), vec![1, 2, 3]);
    }

    #[test]
    fn test_make_cache() {
        use std::cell::Cell;
        let call_count = Cell::new(0);

        let mut cached_square = make_cache(|&x: &i32| {
            call_count.set(call_count.get() + 1);
            x * x
        });

        assert_eq!(cached_square(5), 25);
        assert_eq!(call_count.get(), 1);

        assert_eq!(cached_square(5), 25); // cached
        assert_eq!(call_count.get(), 1);

        assert_eq!(cached_square(3), 9);
        assert_eq!(call_count.get(), 2);
    }

    #[test]
    fn test_make_logged_fn() {
        let double = make_logged_fn(|x: i32| x * 2, "double");
        assert_eq!(double(21), 42);
    }

    #[test]
    fn test_complex_environment() {
        let multiplier = 10;
        let offset = 5;
        let items = vec![1, 2, 3];

        // Closure capturing multiple values
        let complex = move |i: usize| items.get(i).map(|x| x * multiplier + offset);

        assert_eq!(complex(0), Some(15));
        assert_eq!(complex(1), Some(25));
        assert_eq!(complex(2), Some(35));
        assert_eq!(complex(3), None);
    }
}
