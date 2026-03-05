// env! and option_env! for build-time values in Rust

// Embed Cargo metadata at compile time
const PKG_NAME: &str = env!("CARGO_PKG_NAME", "unknown");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION", "0.0.0");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS", "unknown");
const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR", ".");

// Optional: may or may not be set
const CUSTOM_API_URL: Option<&str> = option_env!("API_URL");
const BUILD_PROFILE: Option<&str> = option_env!("PROFILE");

// Embed the full build string
macro_rules! build_info {
    () => {
        concat!(
            env!("CARGO_PKG_NAME", "unknown"),
            " v",
            env!("CARGO_PKG_VERSION", "0.0.0"),
            " (compiled with rustc)"
        )
    };
}

// Build timestamp (set externally during CI)
const BUILD_TIMESTAMP: Option<&str> = option_env!("BUILD_TIMESTAMP");

fn version_info() -> String {
    format!(
        "{} v{} by {}",
        PKG_NAME, PKG_VERSION, PKG_AUTHORS
    )
}

fn api_url() -> &'static str {
    CUSTOM_API_URL.unwrap_or("https://api.example.com")
}

fn main() {
    println!("=== Build-time info ===");
    println!("Package: {}", PKG_NAME);
    println!("Version: {}", PKG_VERSION);
    println!("Authors: {}", PKG_AUTHORS);
    println!("Manifest dir: {}", MANIFEST_DIR);
    println!();

    println!("=== Build string ===");
    println!("{}", build_info!());
    println!("{}", version_info());
    println!();

    println!("=== Optional env vars ===");
    println!("API URL: {}", api_url());
    println!("Build profile: {:?}", BUILD_PROFILE);
    println!("Build timestamp: {:?}", BUILD_TIMESTAMP);
    println!();

    // Runtime env vars (different from compile-time)
    println!("=== Runtime env ===");
    match std::env::var("HOME") {
        Ok(home) => println!("HOME: {}", home),
        Err(_) => println!("HOME not set"),
    }

    // Print all CARGO_ vars (available at runtime too)
    let cargo_vars: Vec<(String, String)> = std::env::vars()
        .filter(|(k, _)| k.starts_with("CARGO_"))
        .take(3)
        .collect();
    for (k, v) in &cargo_vars {
        println!("  {}={}", k, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkg_name_not_empty() {
        // PKG_NAME should be set by Cargo
        // Even if the macro falls back, it won't be empty string from cargo
        assert!(!PKG_NAME.is_empty() || PKG_NAME == "unknown");
    }

    #[test]
    fn test_api_url_fallback() {
        // Without API_URL env var, should use default
        let url = api_url();
        assert!(url.starts_with("http"));
    }
}
