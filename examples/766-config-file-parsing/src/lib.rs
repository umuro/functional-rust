#![allow(clippy::all)]
//! # Config File Parsing
//!
//! INI-style configuration file parser.

use std::collections::HashMap;

/// A configuration with sections
#[derive(Debug, Default)]
pub struct Config {
    pub global: HashMap<String, String>,
    pub sections: HashMap<String, HashMap<String, String>>,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    /// Get a global value
    pub fn get(&self, key: &str) -> Option<&str> {
        self.global.get(key).map(String::as_str)
    }

    /// Get a value from a section
    pub fn get_section(&self, section: &str, key: &str) -> Option<&str> {
        self.sections
            .get(section)
            .and_then(|s| s.get(key))
            .map(String::as_str)
    }

    /// Get all keys in a section
    pub fn section_keys(&self, section: &str) -> Vec<&str> {
        self.sections
            .get(section)
            .map(|s| s.keys().map(String::as_str).collect())
            .unwrap_or_default()
    }
}

/// Parse error
#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidLine { line: usize, content: String },
    DuplicateSection { name: String },
}

/// Parse INI-style config
pub fn parse_config(input: &str) -> Result<Config, ParseError> {
    let mut config = Config::new();
    let mut current_section: Option<String> = None;

    for (line_num, line) in input.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        // Section header
        if line.starts_with('[') && line.ends_with(']') {
            let name = line[1..line.len() - 1].trim().to_string();
            if config.sections.contains_key(&name) {
                return Err(ParseError::DuplicateSection { name });
            }
            config.sections.insert(name.clone(), HashMap::new());
            current_section = Some(name);
            continue;
        }

        // Key-value pair
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().to_string();

            match &current_section {
                Some(section) => {
                    config.sections.get_mut(section).unwrap().insert(key, value);
                }
                None => {
                    config.global.insert(key, value);
                }
            }
        } else {
            return Err(ParseError::InvalidLine {
                line: line_num,
                content: line.to_string(),
            });
        }
    }

    Ok(config)
}

/// Format config back to string
pub fn format_config(config: &Config) -> String {
    let mut output = String::new();

    // Global section
    for (key, value) in &config.global {
        output.push_str(&format!("{} = {}\n", key, value));
    }

    if !config.global.is_empty() && !config.sections.is_empty() {
        output.push('\n');
    }

    // Named sections
    for (section_name, section) in &config.sections {
        output.push_str(&format!("[{}]\n", section_name));
        for (key, value) in section {
            output.push_str(&format!("{} = {}\n", key, value));
        }
        output.push('\n');
    }

    output.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_global() {
        let input = "key = value";
        let config = parse_config(input).unwrap();
        assert_eq!(config.get("key"), Some("value"));
    }

    #[test]
    fn test_parse_section() {
        let input = "[database]\nhost = localhost\nport = 5432";
        let config = parse_config(input).unwrap();
        assert_eq!(config.get_section("database", "host"), Some("localhost"));
        assert_eq!(config.get_section("database", "port"), Some("5432"));
    }

    #[test]
    fn test_parse_comments() {
        let input = "# comment\nkey = value\n; another comment";
        let config = parse_config(input).unwrap();
        assert_eq!(config.get("key"), Some("value"));
        assert_eq!(config.global.len(), 1);
    }

    #[test]
    fn test_duplicate_section() {
        let input = "[section]\na = 1\n[section]\nb = 2";
        let result = parse_config(input);
        assert!(matches!(result, Err(ParseError::DuplicateSection { .. })));
    }

    #[test]
    fn test_invalid_line() {
        let input = "not a valid line";
        let result = parse_config(input);
        assert!(matches!(result, Err(ParseError::InvalidLine { .. })));
    }

    #[test]
    fn test_section_keys() {
        let input = "[server]\nhost = localhost\nport = 8080";
        let config = parse_config(input).unwrap();
        let keys = config.section_keys("server");
        assert!(keys.contains(&"host"));
        assert!(keys.contains(&"port"));
    }
}
