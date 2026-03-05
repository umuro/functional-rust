//! # 536. 'static Lifetime
//! Program-duration references: literals, statics, and 'static bounds.

/// String literals are &'static str — embedded in binary
static APP_NAME: &str = "MyRustApp";
static VERSION: &str = "1.0.0";
static MAX_CONNECTIONS: usize = 100;

/// Static slice
static ERROR_MESSAGES: &[(u16, &str)] = &[
    (404, "Not Found"),
    (500, "Internal Server Error"),
    (403, "Forbidden"),
    (200, "OK"),
];

fn get_error_msg(code: u16) -> &'static str {
    ERROR_MESSAGES.iter()
        .find(|&&(c, _)| c == code)
        .map(|(_, msg)| *msg)
        .unwrap_or("Unknown Error")
}

/// Requires T: 'static — T must own its data (no borrowed refs)
fn store_globally<T: 'static + std::fmt::Debug>(value: T) {
    // In real code, might store in a global Mutex<Vec<Box<dyn Any>>>
    println!("Storing global: {:?}", value);
}

/// Thread spawning requires 'static — closure must not borrow local data
fn spawn_with_static_data() {
    let data = vec![1, 2, 3]; // owned — satisfies 'static
    let handle = std::thread::spawn(move || {
        // data moved in — no borrowed references
        println!("Thread data: {:?}", data);
        data.iter().sum::<i32>()
    });
    println!("Thread result: {}", handle.join().unwrap());
}

/// What 'static actually means for bounded types
fn demonstrate_static_bound() {
    // String: owns data — satisfies T: 'static
    store_globally(String::from("owned string"));

    // i32: no references at all — satisfies T: 'static
    store_globally(42i32);

    // Vec<String>: owns all data — satisfies T: 'static
    store_globally(vec!["a".to_string(), "b".to_string()]);

    // &'static str — is itself 'static
    store_globally("literal string"); // &'static str satisfies 'static

    // &String would NOT satisfy 'static (unless &'static String)
}

/// Lazy static equivalent using OnceLock
use std::sync::OnceLock;
static GLOBAL_CONFIG: OnceLock<Vec<String>> = OnceLock::new();

fn get_config() -> &'static [String] {
    GLOBAL_CONFIG.get_or_init(|| {
        vec!["setting1".to_string(), "setting2".to_string()]
    })
}

fn main() {
    println!("App: {} v{}", APP_NAME, VERSION);
    println!("Max connections: {}", MAX_CONNECTIONS);
    println!("Error 404: {}", get_error_msg(404));
    println!("Error 418: {}", get_error_msg(418));

    println!("\n=== Thread with 'static data ===");
    spawn_with_static_data();

    println!("\n=== 'static bounds ===");
    demonstrate_static_bound();

    println!("\n=== Lazy static ===");
    let config = get_config();
    println!("Config: {:?}", config);
    let config2 = get_config(); // same allocation
    println!("Config2 (same): {:?}", config2);

    // 'static string references
    let literal: &'static str = "I live forever";
    let also_static = APP_NAME;
    println!("\nLiteral: {}, static: {}", literal, also_static);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_error_msg() {
        assert_eq!(get_error_msg(404), "Not Found");
        assert_eq!(get_error_msg(200), "OK");
        assert_eq!(get_error_msg(999), "Unknown Error");
    }

    #[test]
    fn test_literal_is_static() {
        let s: &'static str = "test";
        assert_eq!(s, "test");
    }

    #[test]
    fn test_get_config() {
        let config = get_config();
        assert!(!config.is_empty());
    }

    #[test]
    fn test_static_string_bound() {
        // String satisfies 'static
        fn needs_static<T: 'static>(_: T) {}
        needs_static(String::from("hello"));
        needs_static(42i32);
        needs_static(vec![1, 2, 3]);
    }
}
