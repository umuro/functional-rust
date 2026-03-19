#![allow(clippy::all)]
// Example 130: Typestate Pattern — State Machines in Types
//
// The typestate pattern uses phantom type parameters to encode state in the type
// system, making invalid state transitions a compile-time error rather than a
// runtime panic. Each state is a zero-sized marker struct; the main struct carries
// a PhantomData<State> field so Rust tracks the state without any runtime cost.

use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Approach 1: Door state machine
// ---------------------------------------------------------------------------

/// Zero-sized marker structs — they carry no data, only type information.
pub struct Open;
pub struct Closed;
pub struct Locked;

/// A door whose valid operations depend entirely on its current state.
/// `Door<Open>`, `Door<Closed>`, and `Door<Locked>` are three *different* types.
pub struct Door<State> {
    pub material: String,
    _state: PhantomData<State>,
}

// --- Open state: can close or walk through, but NOT lock directly ---
impl Door<Open> {
    pub fn new(material: &str) -> Self {
        Door {
            material: material.to_string(),
            _state: PhantomData,
        }
    }

    /// Consuming `self` ensures the old `Door<Open>` can no longer be used.
    pub fn close(self) -> Door<Closed> {
        Door {
            material: self.material,
            _state: PhantomData,
        }
    }

    pub fn walk_through(&self) -> String {
        format!("Walking through {} door", self.material)
    }
}

// --- Closed state: can open or lock, but NOT walk through ---
impl Door<Closed> {
    pub fn open(self) -> Door<Open> {
        Door {
            material: self.material,
            _state: PhantomData,
        }
    }

    pub fn lock(self) -> Door<Locked> {
        Door {
            material: self.material,
            _state: PhantomData,
        }
    }
}

// --- Locked state: can only unlock ---
impl Door<Locked> {
    pub fn unlock(self) -> Door<Closed> {
        Door {
            material: self.material,
            _state: PhantomData,
        }
    }
}

// All states share the `state_name` helper via a blanket trait.
pub trait StateName {
    fn state_name(&self) -> &'static str;
}

impl StateName for Door<Open> {
    fn state_name(&self) -> &'static str {
        "open"
    }
}
impl StateName for Door<Closed> {
    fn state_name(&self) -> &'static str {
        "closed"
    }
}
impl StateName for Door<Locked> {
    fn state_name(&self) -> &'static str {
        "locked"
    }
}

// ---------------------------------------------------------------------------
// Approach 2: Database connection state machine
// ---------------------------------------------------------------------------
// Models: Disconnected → Connected → Authenticated → (query allowed)

pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

pub struct DbConnection<State> {
    pub host: String,
    _state: PhantomData<State>,
}

impl DbConnection<Disconnected> {
    pub fn new(host: &str) -> Self {
        DbConnection {
            host: host.to_string(),
            _state: PhantomData,
        }
    }

    pub fn connect(self) -> DbConnection<Connected> {
        DbConnection {
            host: self.host,
            _state: PhantomData,
        }
    }
}

impl DbConnection<Connected> {
    pub fn authenticate(self, _password: &str) -> DbConnection<Authenticated> {
        DbConnection {
            host: self.host,
            _state: PhantomData,
        }
    }
}

impl DbConnection<Authenticated> {
    /// Only callable once you have proved (at compile time) that you authenticated.
    pub fn query(&self, sql: &str) -> String {
        format!("Executing '{}' on {}", sql, self.host)
    }

    pub fn disconnect(self) -> DbConnection<Disconnected> {
        DbConnection {
            host: self.host,
            _state: PhantomData,
        }
    }
}

// ---------------------------------------------------------------------------
// Approach 3: Builder typestate — HTTP request builder
// ---------------------------------------------------------------------------
// Ensures a URL is set before the request can be sent.

pub struct NoUrl;
pub struct HasUrl;

pub struct HttpRequest<UrlState> {
    url: Option<String>,
    body: Option<String>,
    _state: PhantomData<UrlState>,
}

impl HttpRequest<NoUrl> {
    pub fn new() -> Self {
        HttpRequest {
            url: None,
            body: None,
            _state: PhantomData,
        }
    }

    pub fn url(self, url: &str) -> HttpRequest<HasUrl> {
        HttpRequest {
            url: Some(url.to_string()),
            body: self.body,
            _state: PhantomData,
        }
    }
}

impl Default for HttpRequest<NoUrl> {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpRequest<HasUrl> {
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    /// Only reachable when a URL has been provided — guaranteed by the type.
    pub fn send(self) -> String {
        let url = self.url.expect("HasUrl guarantees url is Some");
        match self.body {
            Some(b) => format!("POST {} with body: {}", url, b),
            None => format!("GET {}", url),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Door tests ---

    #[test]
    fn test_door_open_close_open() {
        let door = Door::<Open>::new("oak");
        assert_eq!(door.state_name(), "open");

        let door = door.close();
        assert_eq!(door.state_name(), "closed");

        let door = door.open();
        assert_eq!(door.state_name(), "open");
    }

    #[test]
    fn test_door_full_cycle_with_lock() {
        let door = Door::<Open>::new("steel");
        let door = door.close();
        let door = door.lock();
        assert_eq!(door.state_name(), "locked");

        let door = door.unlock();
        assert_eq!(door.state_name(), "closed");

        let door = door.open();
        let msg = door.walk_through();
        assert_eq!(msg, "Walking through steel door");
    }

    #[test]
    fn test_door_material_preserved_across_transitions() {
        let door = Door::<Open>::new("mahogany");
        let closed = door.close();
        assert_eq!(closed.material, "mahogany");

        let locked = closed.lock();
        assert_eq!(locked.material, "mahogany");

        let closed2 = locked.unlock();
        assert_eq!(closed2.material, "mahogany");
    }

    // --- DbConnection tests ---

    #[test]
    fn test_db_connection_full_flow() {
        let conn = DbConnection::<Disconnected>::new("localhost:5432");
        let conn = conn.connect();
        let conn = conn.authenticate("secret");
        let result = conn.query("SELECT 1");
        assert_eq!(result, "Executing 'SELECT 1' on localhost:5432");

        let _disconnected = conn.disconnect();
    }

    #[test]
    fn test_db_connection_host_preserved() {
        let host = "db.example.com:5432";
        let conn = DbConnection::<Disconnected>::new(host)
            .connect()
            .authenticate("pw");
        assert_eq!(conn.host, host);
    }

    // --- HttpRequest tests ---

    #[test]
    fn test_http_get_request() {
        let result = HttpRequest::new().url("https://example.com/api").send();
        assert_eq!(result, "GET https://example.com/api");
    }

    #[test]
    fn test_http_post_request_with_body() {
        let result = HttpRequest::new()
            .url("https://example.com/api")
            .body(r#"{"key":"value"}"#)
            .send();
        assert_eq!(
            result,
            r#"POST https://example.com/api with body: {"key":"value"}"#
        );
    }

    #[test]
    fn test_http_default_is_no_url() {
        // HttpRequest<NoUrl>::default() compiles; calling .send() would not.
        let req: HttpRequest<NoUrl> = HttpRequest::default();
        // We can only call .url() on it, not .send().
        let result = req.url("https://example.com").send();
        assert_eq!(result, "GET https://example.com");
    }
}
