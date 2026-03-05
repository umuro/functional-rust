//! # Integration Test Structure
//!
//! Demonstrates the pattern for organizing integration tests in Rust projects.
//! Integration tests live in `tests/` directory and test the public API.

/// Configuration for a service
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
}

impl Config {
    /// Create a new configuration
    pub fn new(host: impl Into<String>, port: u16, max_connections: u32) -> Self {
        Config {
            host: host.into(),
            port,
            max_connections,
        }
    }

    /// Create default configuration
    pub fn default_config() -> Self {
        Config::new("localhost", 8080, 100)
    }

    /// Format as address string
    pub fn to_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Configuration validation errors
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    PortOutOfRange(u16),
    EmptyHost,
    InvalidMaxConnections,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::PortOutOfRange(p) => write!(f, "port {} is invalid (use 1-65535)", p),
            ConfigError::EmptyHost => write!(f, "host cannot be empty"),
            ConfigError::InvalidMaxConnections => write!(f, "max_connections must be > 0"),
        }
    }
}

/// Validate a configuration
pub fn validate_config(c: &Config) -> Result<(), ConfigError> {
    if c.host.is_empty() {
        return Err(ConfigError::EmptyHost);
    }
    if c.port == 0 {
        return Err(ConfigError::PortOutOfRange(0));
    }
    if c.max_connections == 0 {
        return Err(ConfigError::InvalidMaxConnections);
    }
    Ok(())
}

/// Parse a port string into a u16
pub fn parse_port(s: &str) -> Result<u16, String> {
    let n: u32 = s
        .parse()
        .map_err(|_| format!("'{}' is not a number", s))?;
    if n == 0 || n > 65535 {
        return Err(format!("port {} out of range (1-65535)", n));
    }
    Ok(n as u16)
}

// Test helpers module - simulates tests/common/mod.rs pattern
pub mod test_helpers {
    use super::*;

    /// Create a test configuration
    pub fn test_config() -> Config {
        Config::new("test-host", 9999, 10)
    }

    /// Assert a configuration is valid
    pub fn assert_valid(c: &Config) {
        assert!(
            validate_config(c).is_ok(),
            "config should be valid: {:?}",
            c
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers::*;

    #[test]
    fn test_default_config_is_valid() {
        let cfg = Config::default_config();
        assert_valid(&cfg);
    }

    #[test]
    fn test_test_config_helper_works() {
        let cfg = test_config();
        assert_eq!(cfg.host, "test-host");
        assert_eq!(cfg.port, 9999);
        assert_valid(&cfg);
    }

    #[test]
    fn test_to_addr_formats_correctly() {
        let cfg = Config::new("example.com", 443, 50);
        assert_eq!(cfg.to_addr(), "example.com:443");
    }

    #[test]
    fn test_empty_host_is_invalid() {
        let cfg = Config::new("", 80, 10);
        assert_eq!(validate_config(&cfg), Err(ConfigError::EmptyHost));
    }

    #[test]
    fn test_zero_port_is_invalid() {
        let cfg = Config::new("localhost", 0, 10);
        assert_eq!(validate_config(&cfg), Err(ConfigError::PortOutOfRange(0)));
    }

    #[test]
    fn test_zero_max_connections_is_invalid() {
        let cfg = Config::new("localhost", 8080, 0);
        assert_eq!(
            validate_config(&cfg),
            Err(ConfigError::InvalidMaxConnections)
        );
    }

    #[test]
    fn test_parse_port_valid() {
        assert_eq!(parse_port("8080"), Ok(8080));
        assert_eq!(parse_port("1"), Ok(1));
        assert_eq!(parse_port("65535"), Ok(65535));
    }

    #[test]
    fn test_parse_port_invalid() {
        assert!(parse_port("0").is_err());
        assert!(parse_port("65536").is_err());
        assert!(parse_port("not_a_number").is_err());
        assert!(parse_port("").is_err());
    }
}
