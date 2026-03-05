//! # 515. Lazy Evaluation with OnceLock
//! Deferred computation using std::sync::OnceLock.

use std::sync::OnceLock;

/// Global lazy value — initialized once on first access
static EXPENSIVE_VALUE: OnceLock<i64> = OnceLock::new();

fn get_expensive_value() -> i64 {
    *EXPENSIVE_VALUE.get_or_init(|| {
        println!("[computing expensive global value...]");
        (1..=1_000_000i64).sum()
    })
}

/// Lazy struct: computes fields only when accessed
struct LazyConfig {
    raw: String,
    // OnceLock lets us cache parsed values without RefCell
    parsed_items: OnceLock<Vec<String>>,
    item_count: OnceLock<usize>,
}

impl LazyConfig {
    fn new(raw: &str) -> Self {
        LazyConfig {
            raw: raw.to_string(),
            parsed_items: OnceLock::new(),
            item_count: OnceLock::new(),
        }
    }

    fn items(&self) -> &[String] {
        self.parsed_items.get_or_init(|| {
            println!("[parsing config...]");
            self.raw.split(',')
                .map(|s| s.trim().to_string())
                .collect()
        })
    }

    fn count(&self) -> usize {
        *self.item_count.get_or_init(|| {
            println!("[counting items...]");
            self.items().len()
        })
    }
}

/// Per-instance OnceLock for lazy computation on structs
struct ExpensiveComputation {
    data: Vec<i32>,
    result: OnceLock<i32>,
}

impl ExpensiveComputation {
    fn new(data: Vec<i32>) -> Self {
        ExpensiveComputation { data, result: OnceLock::new() }
    }

    fn compute(&self) -> i32 {
        *self.result.get_or_init(|| {
            println!("[running expensive computation on {} items]", self.data.len());
            self.data.iter().map(|&x| x * x).sum()
        })
    }
}

fn main() {
    // Global lazy value
    println!("Before first access:");
    let v1 = get_expensive_value();
    let v2 = get_expensive_value(); // instant — already computed
    println!("Value (twice): {} == {}", v1, v2);

    // Lazy struct fields
    println!("\nLazy config:");
    let config = LazyConfig::new("alpha, beta, gamma, delta");
    println!("Accessing count (triggers parse)...");
    println!("Count: {}", config.count());
    println!("Count again (cached): {}", config.count());
    println!("Items: {:?}", config.items()); // already parsed

    // Per-instance lazy computation
    println!("\nPer-instance lazy:");
    let comp = ExpensiveComputation::new(vec![1, 2, 3, 4, 5]);
    println!("First call:");
    println!("Result: {}", comp.compute()); // 1+4+9+16+25=55
    println!("Second call (cached):");
    println!("Result: {}", comp.compute());

    // Thread-safe: OnceLock works across threads
    let comp = std::sync::Arc::new(ExpensiveComputation::new(vec![10, 20, 30]));
    let comp2 = comp.clone();
    let h = std::thread::spawn(move || comp2.compute());
    let r1 = comp.compute();
    let r2 = h.join().unwrap();
    println!("\nThread-safe: {} == {}", r1, r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_once_lock() {
        let v1 = get_expensive_value();
        let v2 = get_expensive_value();
        assert_eq!(v1, v2);
        assert_eq!(v1, 500_000_500_000);
    }

    #[test]
    fn test_lazy_config() {
        let c = LazyConfig::new("a, b, c");
        assert_eq!(c.count(), 3);
        assert_eq!(c.items(), &["a", "b", "c"]);
    }

    #[test]
    fn test_lazy_computation() {
        let c = ExpensiveComputation::new(vec![3, 4]);
        assert_eq!(c.compute(), 25); // 9 + 16
        assert_eq!(c.compute(), 25); // cached
    }
}
