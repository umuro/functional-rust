//! Default Trait
//!
//! Providing default values for types — derivable or custom.

use std::collections::HashMap;

/// Server configuration with derived Default (all zeros/empty).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
    pub debug: bool,
    pub timeout_secs: f64,
}

/// Application configuration with custom Default.
#[derive(Debug, Clone, PartialEq)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
    pub debug: bool,
    pub timeout_secs: f64,
    pub retry_count: u8,
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

impl AppConfig {
    /// Creates a new config with custom port.
    pub fn with_port(port: u16) -> Self {
        AppConfig {
            port,
            ..Default::default()
        }
    }

    /// Creates a debug-enabled config.
    pub fn debug() -> Self {
        AppConfig {
            debug: true,
            ..Default::default()
        }
    }
}

/// A simple counter with Default initialization.
#[derive(Debug, Default, Clone)]
pub struct Counter {
    pub count: u64,
    pub sum: u64,
}

impl Counter {
    pub fn new() -> Self {
        Counter::default()
    }

    pub fn increment(&mut self, value: u64) {
        self.count += 1;
        self.sum += value;
    }

    pub fn average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum as f64 / self.count as f64
        }
    }
}

/// Counts word occurrences using or_default().
pub fn count_words(words: &[&str]) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in words {
        *counts.entry(word.to_string()).or_default() += 1;
    }
    counts
}

/// Gets value from Option or Default.
pub fn get_or_default<T: Default>(opt: Option<T>) -> T {
    opt.unwrap_or_default()
}

/// Generic function requiring Default bound.
pub fn default_if_empty<T: Default>(value: Option<T>) -> T {
    value.unwrap_or_default()
}

/// A builder pattern using Default.
#[derive(Debug, Clone)]
pub struct RequestBuilder {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub timeout_ms: u64,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        RequestBuilder {
            method: "GET".to_string(),
            url: String::new(),
            headers: HashMap::new(),
            timeout_ms: 5000,
        }
    }
}

impl RequestBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let cfg = ServerConfig::default();
        assert_eq!(cfg.host, "");
        assert_eq!(cfg.port, 0);
        assert_eq!(cfg.max_connections, 0);
        assert!(!cfg.debug);
        assert_eq!(cfg.timeout_secs, 0.0);
    }

    #[test]
    fn test_app_config_custom_default() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.host, "localhost");
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.max_connections, 100);
        assert!(!cfg.debug);
        assert_eq!(cfg.timeout_secs, 30.0);
        assert_eq!(cfg.retry_count, 3);
    }

    #[test]
    fn test_struct_update_syntax() {
        let cfg = AppConfig {
            port: 9000,
            debug: true,
            ..Default::default()
        };
        assert_eq!(cfg.port, 9000);
        assert!(cfg.debug);
        assert_eq!(cfg.host, "localhost"); // from default
    }

    #[test]
    fn test_counter_default() {
        let mut c = Counter::default();
        assert_eq!(c.count, 0);
        assert_eq!(c.sum, 0);
        c.increment(10);
        c.increment(20);
        assert_eq!(c.count, 2);
        assert_eq!(c.sum, 30);
        assert_eq!(c.average(), 15.0);
    }

    #[test]
    fn test_count_words() {
        let words = vec!["hello", "world", "hello", "rust"];
        let counts = count_words(&words);
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.get("rust"), Some(&1));
    }

    #[test]
    fn test_unwrap_or_default() {
        let some_vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);
        let none_vec: Option<Vec<i32>> = None;
        
        assert_eq!(get_or_default(some_vec), vec![1, 2, 3]);
        assert_eq!(get_or_default(none_vec), Vec::<i32>::new());
    }

    #[test]
    fn test_request_builder() {
        let req = RequestBuilder::new()
            .method("POST")
            .url("https://api.example.com")
            .header("Content-Type", "application/json")
            .timeout(10000);
        
        assert_eq!(req.method, "POST");
        assert_eq!(req.url, "https://api.example.com");
        assert_eq!(req.headers.get("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(req.timeout_ms, 10000);
    }

    #[test]
    fn test_default_builder() {
        let req = RequestBuilder::default();
        assert_eq!(req.method, "GET");
        assert!(req.url.is_empty());
        assert!(req.headers.is_empty());
        assert_eq!(req.timeout_ms, 5000);
    }
}
