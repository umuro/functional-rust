//! # 524. Builder Pattern with Closures
//! Closure-based configuration in builder APIs.

struct ServerConfig {
    host: String,
    port: u16,
    max_connections: usize,
    timeout_ms: u64,
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
            on_connect: Box::new(|addr| println!("Connected: {}", addr)),
        }
    }
}

struct ServerBuilder {
    config: ServerConfig,
}

impl ServerBuilder {
    fn new() -> Self { ServerBuilder { config: ServerConfig::default() } }

    fn host(mut self, h: &str) -> Self { self.config.host = h.to_string(); self }
    fn port(mut self, p: u16) -> Self { self.config.port = p; self }
    fn max_connections(mut self, n: usize) -> Self { self.config.max_connections = n; self }
    fn timeout_ms(mut self, t: u64) -> Self { self.config.timeout_ms = t; self }

    /// Closure-based configuration step
    fn configure(mut self, f: impl FnOnce(&mut ServerConfig)) -> Self {
        f(&mut self.config);
        self
    }

    /// Closure for the connect handler
    fn on_connect(mut self, handler: impl Fn(&str) + 'static) -> Self {
        self.config.on_connect = Box::new(handler);
        self
    }

    fn build(self) -> ServerConfig { self.config }
}

/// Alternative: configure function takes closure over builder
fn build_server(configure: impl FnOnce(ServerBuilder) -> ServerBuilder) -> ServerConfig {
    configure(ServerBuilder::new()).build()
}

fn main() {
    // Method chain style
    let cfg = ServerBuilder::new()
        .host("0.0.0.0")
        .port(9090)
        .max_connections(500)
        .on_connect(|addr| println!("Custom: {} connected", addr))
        .build();

    println!("Config: {}:{} (max={}, timeout={}ms)",
        cfg.host, cfg.port, cfg.max_connections, cfg.timeout_ms);
    (cfg.on_connect)("192.168.1.1");

    // Closure configure style
    let log_prefix = "SERVER";
    let cfg2 = build_server(|b| b
        .port(3000)
        .configure(|c| { c.max_connections = 50; c.timeout_ms = 1000; })
        .on_connect(move |addr| println!("[{}] {}", log_prefix, addr))
    );
    println!("\nConfig2: {}:{} (max={})", cfg2.host, cfg2.port, cfg2.max_connections);
    (cfg2.on_connect)("10.0.0.1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_defaults() {
        let cfg = ServerBuilder::new().build();
        assert_eq!(cfg.host, "localhost");
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.max_connections, 100);
    }

    #[test]
    fn test_builder_overrides() {
        let cfg = ServerBuilder::new().port(3000).max_connections(50).build();
        assert_eq!(cfg.port, 3000);
        assert_eq!(cfg.max_connections, 50);
    }

    #[test]
    fn test_configure_closure() {
        let cfg = ServerBuilder::new()
            .configure(|c| { c.port = 7777; c.timeout_ms = 100; })
            .build();
        assert_eq!(cfg.port, 7777);
        assert_eq!(cfg.timeout_ms, 100);
    }
}
