//! cfg! and Conditional Compilation
//!
//! Compile-time feature flags and platform-specific code.

/// Platform-specific path separator.
pub fn path_separator() -> char {
    if cfg!(windows) {
        '\\'
    } else {
        '/'
    }
}

/// Debug-only function.
#[cfg(debug_assertions)]
pub fn debug_log(msg: &str) {
    println!("[DEBUG] {}", msg);
}

#[cfg(not(debug_assertions))]
pub fn debug_log(_msg: &str) {
    // No-op in release
}

/// Feature-gated functionality.
#[cfg(feature = "advanced")]
pub fn advanced_feature() -> &'static str {
    "Advanced feature enabled"
}

#[cfg(not(feature = "advanced"))]
pub fn advanced_feature() -> &'static str {
    "Advanced feature disabled"
}

/// OS-specific behavior.
pub fn os_name() -> &'static str {
    #[cfg(target_os = "linux")]
    return "Linux";

    #[cfg(target_os = "macos")]
    return "macOS";

    #[cfg(target_os = "windows")]
    return "Windows";

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return "Unknown OS";
}

/// Architecture info.
pub fn arch_info() -> &'static str {
    if cfg!(target_arch = "x86_64") {
        "64-bit x86"
    } else if cfg!(target_arch = "aarch64") {
        "64-bit ARM"
    } else if cfg!(target_arch = "x86") {
        "32-bit x86"
    } else {
        "Other architecture"
    }
}

/// Test-only utilities.
#[cfg(test)]
pub fn test_helper() -> i32 {
    42
}

/// Conditional derive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub value: i32,
}

/// Platform-specific default.
impl Default for Config {
    fn default() -> Self {
        Config {
            name: if cfg!(debug_assertions) {
                "debug".to_string()
            } else {
                "release".to_string()
            },
            value: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_separator() {
        let sep = path_separator();
        assert!(sep == '/' || sep == '\\');
    }

    #[test]
    fn test_os_name() {
        let name = os_name();
        assert!(!name.is_empty());
    }

    #[test]
    fn test_arch_info() {
        let arch = arch_info();
        assert!(!arch.is_empty());
    }

    #[test]
    fn test_debug_assertions() {
        // This test itself runs in debug mode
        #[cfg(debug_assertions)]
        assert!(true);
    }

    #[test]
    fn test_config_default() {
        let cfg = Config::default();
        #[cfg(debug_assertions)]
        assert_eq!(cfg.name, "debug");
    }

    #[test]
    fn test_test_helper() {
        assert_eq!(test_helper(), 42);
    }
}
