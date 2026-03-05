/// 735: Typestate Builder — required fields enforced at compile time

use std::marker::PhantomData;

// ── Type-level boolean markers ────────────────────────────────────────────────

pub struct Set;
pub struct Unset;

// ── The config we're building ─────────────────────────────────────────────────

pub struct HttpClient {
    pub host: String,
    pub port: u16,
    pub timeout_ms: u64,
    pub max_retries: u32,
}

// ── Builder with phantom state for each required field ────────────────────────

/// `HasHost` and `HasPort` track whether required fields were set.
pub struct HttpClientBuilder<HasHost, HasPort> {
    host:        Option<String>,
    port:        Option<u16>,
    timeout_ms:  u64,
    max_retries: u32,
    _phantom:    PhantomData<(HasHost, HasPort)>,
}

/// Entry point — start with both required fields unset.
impl HttpClientBuilder<Unset, Unset> {
    pub fn new() -> Self {
        HttpClientBuilder {
            host:        None,
            port:        None,
            timeout_ms:  5_000,
            max_retries: 3,
            _phantom:    PhantomData,
        }
    }
}

/// Setting `host` transitions `HasHost` from `Unset` → `Set`.
impl<HasPort> HttpClientBuilder<Unset, HasPort> {
    pub fn host(self, h: impl Into<String>) -> HttpClientBuilder<Set, HasPort> {
        HttpClientBuilder {
            host:        Some(h.into()),
            port:        self.port,
            timeout_ms:  self.timeout_ms,
            max_retries: self.max_retries,
            _phantom:    PhantomData,
        }
    }
}

/// Setting `port` transitions `HasPort` from `Unset` → `Set`.
impl<HasHost> HttpClientBuilder<HasHost, Unset> {
    pub fn port(self, p: u16) -> HttpClientBuilder<HasHost, Set> {
        HttpClientBuilder {
            host:        self.host,
            port:        Some(p),
            timeout_ms:  self.timeout_ms,
            max_retries: self.max_retries,
            _phantom:    PhantomData,
        }
    }
}

/// Optional setters available in any state.
impl<HasHost, HasPort> HttpClientBuilder<HasHost, HasPort> {
    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }
}

/// `build()` ONLY exists when BOTH required fields are `Set`.
impl HttpClientBuilder<Set, Set> {
    pub fn build(self) -> HttpClient {
        HttpClient {
            host:        self.host.unwrap(),
            port:        self.port.unwrap(),
            timeout_ms:  self.timeout_ms,
            max_retries: self.max_retries,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_build_succeeds() {
        let c = HttpClientBuilder::new()
            .host("localhost")
            .port(3000)
            .build();
        assert_eq!(c.host, "localhost");
        assert_eq!(c.port, 3000);
        assert_eq!(c.timeout_ms, 5_000);   // default
    }

    #[test]
    fn custom_timeout_and_retries() {
        let c = HttpClientBuilder::new()
            .host("example.com")
            .port(443)
            .timeout_ms(1_000)
            .max_retries(0)
            .build();
        assert_eq!(c.timeout_ms, 1_000);
        assert_eq!(c.max_retries, 0);
    }

    #[test]
    fn order_of_host_port_does_not_matter() {
        let c1 = HttpClientBuilder::new().host("a").port(1).build();
        let c2 = HttpClientBuilder::new().port(1).host("a").build();
        assert_eq!(c1.host, c2.host);
        assert_eq!(c1.port, c2.port);
    }
}
