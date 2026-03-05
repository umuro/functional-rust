// Example 180: PhantomData for API Safety
// Connection<Closed> vs Connection<Open> — can't query a closed connection

use std::marker::PhantomData;

// === Approach 1: Type-state pattern with PhantomData ===

struct Closed;
struct Open;

struct Connection<State> {
    host: String,
    _state: PhantomData<State>,
}

impl Connection<Closed> {
    fn new(host: &str) -> Self {
        Connection { host: host.to_string(), _state: PhantomData }
    }

    fn open(self) -> Connection<Open> {
        println!("Connecting to {}...", self.host);
        Connection { host: self.host, _state: PhantomData }
    }
}

impl Connection<Open> {
    fn query(&self, sql: &str) -> String {
        format!("result({}): {}", self.host, sql)
    }

    fn execute(&self, sql: &str) -> usize {
        println!("Execute on {}: {}", self.host, sql);
        1 // rows affected
    }

    fn close(self) -> Connection<Closed> {
        println!("Closing {}", self.host);
        Connection { host: self.host, _state: PhantomData }
    }
}

// host() available in any state
impl<S> Connection<S> {
    fn host(&self) -> &str { &self.host }
}

// === Approach 2: Builder pattern with type states ===

struct Disconnected;
struct Connected;
struct InTransaction;

struct DbSession<State> {
    url: String,
    _state: PhantomData<State>,
}

impl DbSession<Disconnected> {
    fn new(url: &str) -> Self {
        DbSession { url: url.to_string(), _state: PhantomData }
    }

    fn connect(self) -> DbSession<Connected> {
        DbSession { url: self.url, _state: PhantomData }
    }
}

impl DbSession<Connected> {
    fn begin_transaction(self) -> DbSession<InTransaction> {
        DbSession { url: self.url, _state: PhantomData }
    }

    fn query(&self, sql: &str) -> String {
        format!("query({}): {}", self.url, sql)
    }

    fn disconnect(self) -> DbSession<Disconnected> {
        DbSession { url: self.url, _state: PhantomData }
    }
}

impl DbSession<InTransaction> {
    fn query(&self, sql: &str) -> String {
        format!("tx_query({}): {}", self.url, sql)
    }

    fn commit(self) -> DbSession<Connected> {
        println!("COMMIT");
        DbSession { url: self.url, _state: PhantomData }
    }

    fn rollback(self) -> DbSession<Connected> {
        println!("ROLLBACK");
        DbSession { url: self.url, _state: PhantomData }
    }
}

// === Approach 3: File handle safety ===

struct Unopened;
struct Opened;

struct SafeFile<State> {
    path: String,
    content: Option<String>,
    _state: PhantomData<State>,
}

impl SafeFile<Unopened> {
    fn new(path: &str) -> Self {
        SafeFile { path: path.to_string(), content: None, _state: PhantomData }
    }

    fn open(self) -> SafeFile<Opened> {
        SafeFile {
            path: self.path,
            content: Some(String::new()),
            _state: PhantomData,
        }
    }
}

impl SafeFile<Opened> {
    fn write(&mut self, data: &str) {
        if let Some(ref mut c) = self.content {
            c.push_str(data);
        }
    }

    fn read(&self) -> &str {
        self.content.as_deref().unwrap_or("")
    }

    fn close(self) -> SafeFile<Unopened> {
        SafeFile { path: self.path, content: None, _state: PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_lifecycle() {
        let conn = Connection::<Closed>::new("db.test");
        assert_eq!(conn.host(), "db.test");
        let conn = conn.open();
        assert_eq!(conn.query("SELECT 1"), "result(db.test): SELECT 1");
        let closed = conn.close();
        assert_eq!(closed.host(), "db.test");
    }

    #[test]
    fn test_db_session() {
        let s = DbSession::new("pg://localhost").connect();
        assert!(s.query("X").contains("query"));
        let tx = s.begin_transaction();
        assert!(tx.query("X").contains("tx_query"));
        let s = tx.commit();
        let _d = s.disconnect();
    }

    #[test]
    fn test_safe_file() {
        let f = SafeFile::<Unopened>::new("test.txt");
        let mut f = f.open();
        f.write("abc");
        f.write("def");
        assert_eq!(f.read(), "abcdef");
        let _closed = f.close();
    }
}
