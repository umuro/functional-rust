//! env! and option_env! Macros
//!
//! Accessing environment variables at compile time.

/// Package version from Cargo.toml.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Package name from Cargo.toml.
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

/// Authors from Cargo.toml.
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// Get version string.
pub fn version() -> &'static str {
    VERSION
}

/// Get full version string.
pub fn full_version() -> String {
    format!("{} v{}", PKG_NAME, VERSION)
}

/// Optional compile-time env var.
pub fn build_profile() -> &'static str {
    option_env!("PROFILE").unwrap_or("unknown")
}

/// Check if debug build.
pub fn is_debug_build() -> bool {
    cfg!(debug_assertions)
}

/// Get optional feature flag.
pub fn optional_api_key() -> Option<&'static str> {
    option_env!("API_KEY")
}

/// Build metadata.
pub struct BuildInfo {
    pub version: &'static str,
    pub name: &'static str,
    pub target: &'static str,
}

impl BuildInfo {
    pub const fn new() -> Self {
        BuildInfo {
            version: env!("CARGO_PKG_VERSION"),
            name: env!("CARGO_PKG_NAME"),
            target: match option_env!("CARGO_CFG_TARGET_ARCH") {
                Some(v) => v,
                None => "unknown",
            },
        }
    }
}

impl Default for BuildInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Compile-time manifest directory.
pub fn manifest_dir() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

/// Include a file relative to manifest.
#[macro_export]
macro_rules! include_asset {
    ($path:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $path))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_pkg_name() {
        assert_eq!(PKG_NAME, "example-420-macro-env");
    }

    #[test]
    fn test_full_version() {
        let fv = full_version();
        assert!(fv.contains(PKG_NAME));
        assert!(fv.contains(VERSION));
    }

    #[test]
    fn test_build_info() {
        let info = BuildInfo::new();
        assert_eq!(info.version, VERSION);
        assert_eq!(info.name, PKG_NAME);
    }

    #[test]
    fn test_manifest_dir() {
        let dir = manifest_dir();
        assert!(!dir.is_empty());
    }

    #[test]
    fn test_option_env_missing() {
        let val = option_env!("VERY_UNLIKELY_ENV_VAR_12345");
        assert!(val.is_none());
    }

    #[test]
    fn test_option_env_present() {
        let val = option_env!("CARGO_PKG_NAME");
        assert!(val.is_some());
    }
}
