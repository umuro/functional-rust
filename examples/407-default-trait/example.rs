// Default trait and initialization in Rust

#[derive(Debug, Default, Clone)]
struct ServerConfig {
    host: String,      // default: ""
    port: u16,         // default: 0
    max_connections: u32, // default: 0
    debug: bool,       // default: false
    timeout_secs: f64, // default: 0.0
}

// Custom Default for non-trivial defaults
#[derive(Debug, Clone)]
struct AppConfig {
    host: String,
    port: u16,
    max_connections: u32,
    debug: bool,
    timeout_secs: f64,
    retry_count: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            host: "localhost".to_string(),
            port: 8080,
            max_connections: 100,
            debug: false,
            timeout_secs: 30.0,
            retry_count: 3,
        }
    }
}

#[derive(Debug, Default)]
struct Counter { count: u64, total: u64 }

impl Counter {
    fn increment(&mut self) { self.count += 1; self.total += 1; }
    fn average(&self) -> f64 { if self.count == 0 { 0.0 } else { self.total as f64 / self.count as f64 } }
}

fn main() {
    // Derive Default
    let config = ServerConfig::default();
    println!("{:?}", config);

    // Struct update syntax with custom Default
    let custom = AppConfig {
        port: 9090,
        debug: true,
        ..AppConfig::default()  // fill the rest with defaults
    };
    println!("{:?}", custom);

    // Using Default in collections
    use std::collections::HashMap;
    let mut word_count: HashMap<&str, u32> = HashMap::new();
    let words = ["hello", "world", "hello", "rust", "world", "hello"];
    for word in &words {
        *word_count.entry(word).or_default() += 1; // or_default() uses u32::default() = 0
    }
    println!("{:?}", word_count);

    // Option::unwrap_or_default
    let opt: Option<Vec<i32>> = None;
    let v = opt.unwrap_or_default();
    println!("unwrap_or_default: {:?}", v); // []

    // Counter with Default
    let mut c = Counter::default();
    for _ in 0..5 { c.increment(); }
    println!("Count: {}, Average: {}", c.count, c.average());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_default() {
        let c = ServerConfig::default();
        assert_eq!(c.host, "");
        assert_eq!(c.port, 0);
        assert!(!c.debug);
    }

    #[test]
    fn test_custom_default() {
        let c = AppConfig::default();
        assert_eq!(c.host, "localhost");
        assert_eq!(c.port, 8080);
        assert_eq!(c.retry_count, 3);
    }

    #[test]
    fn test_struct_update() {
        let c = AppConfig { port: 9000, ..AppConfig::default() };
        assert_eq!(c.port, 9000);
        assert_eq!(c.host, "localhost"); // from default
    }
}
