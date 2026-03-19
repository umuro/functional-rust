#![allow(clippy::all)]
//! # Functional Builder Pattern
//!
//! Builder pattern using consuming methods that return Self for chaining.

/// Network connection configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub timeout: f64,
    pub retries: u32,
    pub tls: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "localhost".into(),
            port: 80,
            timeout: 30.0,
            retries: 3,
            tls: false,
        }
    }
}

impl Config {
    /// Create a new config with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the host (consuming builder method).
    pub fn host(mut self, h: impl Into<String>) -> Self {
        self.host = h.into();
        self
    }

    /// Set the port.
    pub fn port(mut self, p: u16) -> Self {
        self.port = p;
        self
    }

    /// Set the timeout in seconds.
    pub fn timeout(mut self, t: f64) -> Self {
        self.timeout = t;
        self
    }

    /// Set the retry count.
    pub fn retries(mut self, r: u32) -> Self {
        self.retries = r;
        self
    }

    /// Enable or disable TLS.
    pub fn tls(mut self, b: bool) -> Self {
        self.tls = b;
        self
    }

    /// Validate and build the final config.
    pub fn build(self) -> Result<Self, String> {
        if self.host.is_empty() {
            return Err("host required".into());
        }
        if self.port == 0 {
            return Err("port required".into());
        }
        if self.timeout <= 0.0 {
            return Err("timeout must be positive".into());
        }
        Ok(self)
    }
}

/// HTTP request builder.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl Request {
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: "GET".into(),
            url: url.into(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: "POST".into(),
            url: url.into(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn json_body(self, body: impl Into<String>) -> Self {
        self.header("Content-Type", "application/json").body(body)
    }
}

/// Query builder for SQL-like queries.
#[derive(Debug, Clone, Default)]
pub struct Query {
    pub table: String,
    pub columns: Vec<String>,
    pub where_clause: Option<String>,
    pub order_by: Option<String>,
    pub limit: Option<usize>,
}

impl Query {
    pub fn from(table: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            ..Default::default()
        }
    }

    pub fn select(mut self, cols: &[&str]) -> Self {
        self.columns = cols.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn where_eq(mut self, clause: impl Into<String>) -> Self {
        self.where_clause = Some(clause.into());
        self
    }

    pub fn order_by(mut self, col: impl Into<String>) -> Self {
        self.order_by = Some(col.into());
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    pub fn to_sql(&self) -> String {
        let cols = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };

        let mut sql = format!("SELECT {} FROM {}", cols, self.table);

        if let Some(ref w) = self.where_clause {
            sql.push_str(&format!(" WHERE {}", w));
        }
        if let Some(ref o) = self.order_by {
            sql.push_str(&format!(" ORDER BY {}", o));
        }
        if let Some(l) = self.limit {
            sql.push_str(&format!(" LIMIT {}", l));
        }

        sql
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let cfg = Config::default()
            .host("api.example.com")
            .port(443)
            .tls(true)
            .timeout(60.0)
            .build()
            .unwrap();

        assert_eq!(cfg.host, "api.example.com");
        assert_eq!(cfg.port, 443);
        assert!(cfg.tls);
    }

    #[test]
    fn test_config_validation_empty_host() {
        let result = Config::default().host("").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_zero_port() {
        let result = Config::default().port(0).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_clone_and_modify() {
        let base = Config::default().retries(5);
        let dev = base.clone().host("dev.local");
        let prod = base.host("prod.example.com").tls(true);

        assert_eq!(dev.host, "dev.local");
        assert_eq!(prod.host, "prod.example.com");
        assert_eq!(dev.retries, 5);
        assert_eq!(prod.retries, 5);
    }

    #[test]
    fn test_request_builder() {
        let req = Request::get("https://api.example.com/users")
            .header("Authorization", "Bearer token")
            .header("Accept", "application/json");

        assert_eq!(req.method, "GET");
        assert_eq!(req.headers.len(), 2);
    }

    #[test]
    fn test_request_json_body() {
        let req = Request::post("https://api.example.com/users").json_body(r#"{"name": "Alice"}"#);

        assert_eq!(req.method, "POST");
        assert!(req
            .headers
            .iter()
            .any(|(k, v)| k == "Content-Type" && v == "application/json"));
        assert!(req.body.is_some());
    }

    #[test]
    fn test_query_builder() {
        let sql = Query::from("users")
            .select(&["id", "name", "email"])
            .where_eq("active = true")
            .order_by("name")
            .limit(10)
            .to_sql();

        assert_eq!(
            sql,
            "SELECT id, name, email FROM users WHERE active = true ORDER BY name LIMIT 10"
        );
    }

    #[test]
    fn test_query_default_columns() {
        let sql = Query::from("users").to_sql();
        assert_eq!(sql, "SELECT * FROM users");
    }
}
