//! # Serde Attributes Concept
//!
//! Understanding serde's attribute system without the actual serde crate.

use std::collections::HashMap;

/// Simulated field attributes
#[derive(Debug, Clone)]
pub struct FieldConfig {
    pub rename: Option<String>,
    pub skip: bool,
    pub default: bool,
    pub flatten: bool,
}

impl Default for FieldConfig {
    fn default() -> Self {
        FieldConfig {
            rename: None,
            skip: false,
            default: false,
            flatten: false,
        }
    }
}

/// A struct with "serde-like" configuration
#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,        // Would have #[serde(skip)]
    pub display_name: Option<String>, // Would have #[serde(default)]
}

impl User {
    pub fn field_configs() -> HashMap<&'static str, FieldConfig> {
        let mut configs = HashMap::new();

        configs.insert("id", FieldConfig::default());
        configs.insert(
            "username",
            FieldConfig {
                rename: Some("user_name".to_string()),
                ..Default::default()
            },
        );
        configs.insert("email", FieldConfig::default());
        configs.insert(
            "password_hash",
            FieldConfig {
                skip: true,
                ..Default::default()
            },
        );
        configs.insert(
            "display_name",
            FieldConfig {
                default: true,
                ..Default::default()
            },
        );

        configs
    }

    /// Serialize to JSON-like string
    pub fn to_json(&self) -> String {
        let configs = Self::field_configs();
        let mut pairs = Vec::new();

        // id
        pairs.push(r#""id": "#.to_owned() + &self.id.to_string());

        // username (renamed to user_name)
        if let Some(cfg) = configs.get("username") {
            let key = cfg.rename.as_deref().unwrap_or("username");
            pairs.push(format!(r#""{}": "{}""#, key, self.username));
        }

        // email
        pairs.push(format!(r#""email": "{}""#, self.email));

        // password_hash - skipped!
        // (not included)

        // display_name (optional with default)
        if let Some(name) = &self.display_name {
            pairs.push(format!(r#""display_name": "{}""#, name));
        }

        format!("{{{}}}", pairs.join(", "))
    }
}

/// Demonstrate rename_all
#[derive(Debug)]
pub struct Config {
    pub database_host: String,
    pub database_port: u16,
    pub max_connections: u32,
}

impl Config {
    /// Serialize with snake_case to camelCase conversion
    pub fn to_camel_case_json(&self) -> String {
        let pairs = vec![
            format!(r#""databaseHost": "{}""#, self.database_host),
            format!(r#""databasePort": {}"#, self.database_port),
            format!(r#""maxConnections": {}"#, self.max_connections),
        ];
        format!("{{{}}}", pairs.join(", "))
    }

    /// Serialize with snake_case to kebab-case
    pub fn to_kebab_case_json(&self) -> String {
        let pairs = vec![
            format!(r#""database-host": "{}""#, self.database_host),
            format!(r#""database-port": {}"#, self.database_port),
            format!(r#""max-connections": {}"#, self.max_connections),
        ];
        format!("{{{}}}", pairs.join(", "))
    }
}

/// Convert snake_case to camelCase
pub fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Convert snake_case to kebab-case
pub fn snake_to_kebab(s: &str) -> String {
    s.replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_to_json() {
        let user = User {
            id: 1,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "secret123".to_string(),
            display_name: Some("Alice".to_string()),
        };
        let json = user.to_json();

        assert!(json.contains(r#""id": 1"#));
        assert!(json.contains(r#""user_name": "alice""#)); // renamed
        assert!(!json.contains("password")); // skipped
        assert!(json.contains(r#""display_name": "Alice""#));
    }

    #[test]
    fn test_config_camel_case() {
        let config = Config {
            database_host: "localhost".to_string(),
            database_port: 5432,
            max_connections: 100,
        };
        let json = config.to_camel_case_json();

        assert!(json.contains("databaseHost"));
        assert!(json.contains("databasePort"));
        assert!(json.contains("maxConnections"));
    }

    #[test]
    fn test_snake_to_camel() {
        assert_eq!(snake_to_camel("hello_world"), "helloWorld");
        assert_eq!(snake_to_camel("database_host"), "databaseHost");
    }

    #[test]
    fn test_snake_to_kebab() {
        assert_eq!(snake_to_kebab("hello_world"), "hello-world");
        assert_eq!(snake_to_kebab("database_host"), "database-host");
    }
}
