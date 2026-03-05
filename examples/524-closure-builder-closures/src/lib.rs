//! Builder Pattern with Closures
//!
//! Closure-based configuration in builder APIs.

/// Server configuration with closure callback.
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub timeout_ms: u64,
    on_connect: Box<dyn Fn(&str)>,
}

impl std::fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("max_connections", &self.max_connections)
            .field("timeout_ms", &self.timeout_ms)
            .field("on_connect", &"<fn>")
            .finish()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: "localhost".to_string(),
            port: 8080,
            max_connections: 100,
            timeout_ms: 5000,
            on_connect: Box::new(|_| {}),
        }
    }
}

/// Builder for ServerConfig.
pub struct ServerBuilder {
    config: ServerConfig,
}

impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder {
            config: ServerConfig::default(),
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.config.host = host.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    pub fn max_connections(mut self, max: usize) -> Self {
        self.config.max_connections = max;
        self
    }

    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.config.timeout_ms = ms;
        self
    }

    pub fn on_connect(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.config.on_connect = Box::new(f);
        self
    }

    pub fn build(self) -> ServerConfig {
        self.config
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerConfig {
    pub fn connect(&self, client: &str) {
        (self.on_connect)(client);
    }
}

/// Request handler builder.
pub struct RequestHandler {
    validators: Vec<Box<dyn Fn(&str) -> Result<(), String>>>,
    transformer: Box<dyn Fn(String) -> String>,
}

impl RequestHandler {
    pub fn new() -> Self {
        RequestHandler {
            validators: Vec::new(),
            transformer: Box::new(|s| s),
        }
    }

    pub fn validate(mut self, f: impl Fn(&str) -> Result<(), String> + 'static) -> Self {
        self.validators.push(Box::new(f));
        self
    }

    pub fn transform(mut self, f: impl Fn(String) -> String + 'static) -> Self {
        self.transformer = Box::new(f);
        self
    }

    pub fn process(&self, input: &str) -> Result<String, String> {
        for validator in &self.validators {
            validator(input)?;
        }
        Ok((self.transformer)(input.to_string()))
    }
}

impl Default for RequestHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_server_builder_defaults() {
        let config = ServerBuilder::new().build();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_server_builder_custom() {
        let config = ServerBuilder::new()
            .host("0.0.0.0")
            .port(3000)
            .max_connections(500)
            .build();

        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.max_connections, 500);
    }

    #[test]
    fn test_server_on_connect() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let log_clone = log.clone();

        let config = ServerBuilder::new()
            .on_connect(move |client| {
                log_clone.borrow_mut().push(client.to_string());
            })
            .build();

        config.connect("client1");
        config.connect("client2");

        assert_eq!(*log.borrow(), vec!["client1", "client2"]);
    }

    #[test]
    fn test_request_handler_valid() {
        let handler = RequestHandler::new()
            .validate(|s| {
                if s.is_empty() {
                    Err("empty input".into())
                } else {
                    Ok(())
                }
            })
            .transform(|s| s.to_uppercase());

        assert_eq!(handler.process("hello").unwrap(), "HELLO");
    }

    #[test]
    fn test_request_handler_invalid() {
        let handler = RequestHandler::new().validate(|s| {
            if s.len() < 3 {
                Err("too short".into())
            } else {
                Ok(())
            }
        });

        assert!(handler.process("ab").is_err());
        assert!(handler.process("abc").is_ok());
    }

    #[test]
    fn test_request_handler_chain() {
        let handler = RequestHandler::new()
            .validate(|s| {
                if s.contains(' ') {
                    Err("no spaces".into())
                } else {
                    Ok(())
                }
            })
            .validate(|s| {
                if s.is_empty() {
                    Err("empty".into())
                } else {
                    Ok(())
                }
            })
            .transform(|s| format!("[{}]", s));

        assert_eq!(handler.process("test").unwrap(), "[test]");
        assert!(handler.process("has space").is_err());
    }
}
