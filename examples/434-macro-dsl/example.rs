// DSL design with macros in Rust

// Config DSL
macro_rules! config {
    ($($key:ident = $value:expr),* $(,)?) => {
        {
            use std::collections::HashMap;
            let mut map: HashMap<String, ConfigValue> = HashMap::new();
            $(map.insert(stringify!($key).to_string(), ConfigValue::from($value));)*
            Config(map)
        }
    };
}

#[derive(Debug, Clone)]
enum ConfigValue {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<ConfigValue>),
}

impl From<&str> for ConfigValue {
    fn from(s: &str) -> Self { ConfigValue::Str(s.to_string()) }
}
impl From<i64> for ConfigValue {
    fn from(n: i64) -> Self { ConfigValue::Int(n) }
}
impl From<i32> for ConfigValue {
    fn from(n: i32) -> Self { ConfigValue::Int(n as i64) }
}
impl From<f64> for ConfigValue {
    fn from(f: f64) -> Self { ConfigValue::Float(f) }
}
impl From<bool> for ConfigValue {
    fn from(b: bool) -> Self { ConfigValue::Bool(b) }
}

use std::collections::HashMap;
struct Config(HashMap<String, ConfigValue>);

impl Config {
    fn get(&self, key: &str) -> Option<&ConfigValue> { self.0.get(key) }
}

// Test assertion DSL
macro_rules! assert_that {
    ($val:expr, equals $expected:expr) => {
        assert_eq!($val, $expected,
            "Expected {} to equal {}", stringify!($val), stringify!($expected));
    };
    ($val:expr, is_some) => {
        assert!($val.is_some(), "Expected {} to be Some, got None", stringify!($val));
    };
    ($val:expr, is_none) => {
        assert!($val.is_none(), "Expected {} to be None", stringify!($val));
    };
    ($val:expr, contains $item:expr) => {
        assert!($val.contains(&$item),
            "Expected {:?} to contain {:?}", $val, $item);
    };
    ($val:expr, has_len $len:expr) => {
        assert_eq!($val.len(), $len,
            "Expected len {} but got {}", $len, $val.len());
    };
}

// Route DSL
macro_rules! router {
    ($($method:ident $path:literal => $handler:expr),* $(,)?) => {
        {
            let routes: Vec<(&str, &str, Box<dyn Fn() -> String>)> = vec![
                $(($method_str!($method), $path, Box::new($handler)),)*
            ];
            routes
        }
    };
}

macro_rules! method_str {
    (GET) => { "GET" };
    (POST) => { "POST" };
    (PUT) => { "PUT" };
    (DELETE) => { "DELETE" };
}

fn main() {
    // Config DSL
    let cfg = config!(
        host = "localhost",
        port = 8080i32,
        debug = true,
        timeout = 30.0f64,
    );
    println!("host: {:?}", cfg.get("host"));
    println!("port: {:?}", cfg.get("port"));
    println!("debug: {:?}", cfg.get("debug"));

    // Route DSL
    let routes: Vec<(&str, &str, Box<dyn Fn() -> String>)> = vec![
        ("GET", "/users", Box::new(|| "[{"id":1}]".to_string())),
        ("POST", "/users", Box::new(|| "created".to_string())),
        ("GET", "/health", Box::new(|| "ok".to_string())),
    ];

    println!("
Routes:");
    for (method, path, handler) in &routes {
        println!("  {} {} -> {}", method, path, handler());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dsl() {
        let cfg = config!(name = "test", value = 42i32);
        assert!(cfg.get("name").is_some());
        assert!(cfg.get("value").is_some());
        assert!(cfg.get("missing").is_none());
    }

    #[test]
    fn test_assert_that() {
        let v = vec![1, 2, 3];
        assert_that!(v.len(), equals 3);
        assert_that!(v, contains 2);
        assert_that!(v, has_len 3);
        let opt: Option<i32> = Some(42);
        assert_that!(opt, is_some);
        let none: Option<i32> = None;
        assert_that!(none, is_none);
    }
}
