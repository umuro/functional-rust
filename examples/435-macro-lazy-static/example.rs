// lazy_static! / once_cell pattern in Rust
use std::sync::{OnceLock, Mutex};
use std::collections::HashMap;

// Modern std approach: LazyLock (stable since Rust 1.80)
// OnceLock is stable since 1.70

// Static with lazy initialization using OnceLock
static GLOBAL_CONFIG: OnceLock<HashMap<String, String>> = OnceLock::new();

fn get_config() -> &'static HashMap<String, String> {
    GLOBAL_CONFIG.get_or_init(|| {
        println!("Initializing config (once)...");
        let mut m = HashMap::new();
        m.insert("host".to_string(), "localhost".to_string());
        m.insert("port".to_string(), "8080".to_string());
        m.insert("debug".to_string(), "false".to_string());
        m
    })
}

// Thread-safe singleton counter
static COUNTER: OnceLock<Mutex<u64>> = OnceLock::new();

fn get_counter() -> &'static Mutex<u64> {
    COUNTER.get_or_init(|| Mutex::new(0))
}

fn increment() -> u64 {
    let mut c = get_counter().lock().unwrap();
    *c += 1;
    *c
}

// Simulate lazy_static! macro (before OnceLock was stable)
macro_rules! lazy_static_sim {
    (static ref $name:ident : $ty:ty = $init:expr ;) => {
        static $name: OnceLock<$ty> = OnceLock::new();
        fn get_lazy_() -> &'static $ty {
            $name.get_or_init(|| $init)
        }
    };
}

// Using LazyLock (Rust 1.80+)
// use std::sync::LazyLock;
// static PRIMES: LazyLock<Vec<u32>> = LazyLock::new(|| {
//     println!("Computing primes...");
//     sieve_of_eratosthenes(100)
// });

fn sieve(limit: usize) -> Vec<u32> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit > 0 { is_prime[1] = false; }
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit { is_prime[j] = false; j += i; }
        }
        i += 1;
    }
    (2..=limit).filter(|&n| is_prime[n]).map(|n| n as u32).collect()
}

static PRIMES_100: OnceLock<Vec<u32>> = OnceLock::new();

fn get_primes() -> &'static [u32] {
    PRIMES_100.get_or_init(|| {
        println!("Computing primes up to 100...");
        sieve(100)
    })
}

fn main() {
    // Config initialized once
    let config = get_config();
    println!("host: {}", config["host"]);
    println!("port: {}", config["port"]);

    // Second access — no re-initialization
    let config2 = get_config();
    println!("host again: {}", config2["host"]);

    // Thread-safe counter
    println!("
Counter: {}", increment());
    println!("Counter: {}", increment());
    println!("Counter: {}", increment());

    // Primes (lazy)
    println!("
Primes <= 100 (first access):");
    let primes = get_primes();
    println!("{:?}", &primes[..10]);

    println!("Primes again (cached):");
    println!("{} primes total", get_primes().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let c = get_config();
        assert_eq!(c["host"], "localhost");
    }

    #[test]
    fn test_primes() {
        let p = get_primes();
        assert_eq!(p[0], 2);
        assert_eq!(p[1], 3);
        assert!(p.contains(&97)); // 97 is prime
    }

    #[test]
    fn test_counter() {
        let a = increment();
        let b = increment();
        assert!(b > a);
    }
}
