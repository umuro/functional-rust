// cfg! and cfg_attr for conditional code in Rust

// Conditional function — only exists on Unix
#[cfg(unix)]
fn platform_info() -> &'static str { "Unix-like (Linux/macOS)" }

#[cfg(windows)]
fn platform_info() -> &'static str { "Windows" }

// Fallback for other platforms
#[cfg(not(any(unix, windows)))]
fn platform_info() -> &'static str { "Unknown platform" }

// Debug-only logging
macro_rules! debug_log {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

// Conditional derives via cfg_attr
#[cfg_attr(test, derive(PartialEq))] // only derive PartialEq in test builds
#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
}

// Feature-gated code (simulated — real features defined in Cargo.toml)
// #[cfg(feature = "json")]
// fn to_json(&self) -> String { ... }

// Conditional test helper
#[cfg(test)]
fn make_test_config() -> Config {
    Config { host: "localhost".to_string(), port: 8080 }
}

// Target-specific code
fn word_size() -> usize {
    if cfg!(target_pointer_width = "64") { 64 }
    else if cfg!(target_pointer_width = "32") { 32 }
    else { 0 }
}

// Endianness
fn endian() -> &'static str {
    if cfg!(target_endian = "big") { "big-endian" }
    else { "little-endian" }
}

fn main() {
    println!("Platform: {}", platform_info());
    println!("Word size: {} bits", word_size());
    println!("Endian: {}", endian());

    debug_log!("This appears in debug builds");
    debug_log!("Value: {}", 42);

    // cfg! as expression
    let is_debug = cfg!(debug_assertions);
    println!("Debug build: {}", is_debug);

    let c = Config { host: "example.com".to_string(), port: 443 };
    println!("{:?}", c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let c = make_test_config();
        assert_eq!(c.host, "localhost");
        assert_eq!(c.port, 8080);
    }

    #[test]
    fn test_cfg_expr() {
        let word_sz = word_size();
        assert!(word_sz == 32 || word_sz == 64 || word_sz == 0);
    }

    // This test only runs on Unix
    #[cfg(unix)]
    #[test]
    fn test_unix_platform() {
        assert_eq!(platform_info(), "Unix-like (Linux/macOS)");
    }
}
