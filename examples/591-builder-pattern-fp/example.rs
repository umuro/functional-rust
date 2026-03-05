#[derive(Debug,Clone)]
struct Config {
    host:    String,
    port:    u16,
    timeout: f64,
    retries: u32,
    tls:     bool,
}

impl Default for Config {
    fn default() -> Self {
        Config { host:"localhost".into(), port:80, timeout:30.0, retries:3, tls:false }
    }
}

// Consuming builder (functional style)
impl Config {
    fn host(mut self, h: impl Into<String>) -> Self { self.host = h.into(); self }
    fn port(mut self, p: u16)               -> Self { self.port = p; self }
    fn timeout(mut self, t: f64)             -> Self { self.timeout = t; self }
    fn retries(mut self, r: u32)             -> Self { self.retries = r; self }
    fn tls(mut self, b: bool)                -> Self { self.tls = b; self }

    fn build(self) -> Result<Self, String> {
        if self.host.is_empty() { return Err("host required".into()); }
        if self.port == 0       { return Err("port required".into()); }
        Ok(self)
    }
}

fn main() {
    let cfg = Config::default()
        .host("api.example.com")
        .port(443)
        .tls(true)
        .timeout(60.0)
        .build()
        .unwrap();
    println!("{:?}", cfg);

    // Reuse a partial config
    let base = Config::default().retries(5);
    let dev  = base.clone().host("dev.local");
    let prod = base.host("prod.example.com").tls(true);
    println!("dev:{}  prod:{}", dev.host, prod.host);

    // Validation
    let bad = Config::default().host("").port(0).build();
    println!("bad: {}", bad.unwrap_err());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn build_ok() {
        let c = Config::default().host("h").port(80).build();
        assert!(c.is_ok());
    }
    #[test] fn build_fail() {
        let c = Config::default().host("").build();
        assert!(c.is_err());
    }
}
