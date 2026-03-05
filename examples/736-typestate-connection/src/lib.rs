/// 736: TCP Connection modelled as typestate
/// Send/recv only available on Connected; connect only on Disconnected.

use std::marker::PhantomData;

// ── State markers ─────────────────────────────────────────────────────────────
pub struct Disconnected;
pub struct Connecting;
pub struct Connected;
pub struct Closed;

// ── Connection ────────────────────────────────────────────────────────────────

pub struct TcpConn<State> {
    host: String,
    port: u16,
    // In a real impl, this would hold a socket fd
    bytes_sent: usize,
    bytes_recv: usize,
    _state: PhantomData<State>,
}

impl TcpConn<Disconnected> {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        TcpConn {
            host: host.into(),
            port,
            bytes_sent: 0,
            bytes_recv: 0,
            _state: PhantomData,
        }
    }

    /// Transition: Disconnected → Connected
    pub fn connect(self) -> Result<TcpConn<Connected>, String> {
        println!("Connecting to {}:{} ...", self.host, self.port);
        // In reality: TcpStream::connect(...)
        Ok(TcpConn {
            host: self.host,
            port: self.port,
            bytes_sent: 0,
            bytes_recv: 0,
            _state: PhantomData,
        })
    }
}

impl TcpConn<Connected> {
    /// Send data — only available when Connected.
    pub fn send(mut self, data: &[u8]) -> Result<TcpConn<Connected>, String> {
        println!("[{}:{}] → {} bytes", self.host, self.port, data.len());
        self.bytes_sent += data.len();
        Ok(self)
    }

    /// Receive data — only available when Connected.
    pub fn recv(mut self) -> Result<(Vec<u8>, TcpConn<Connected>), String> {
        let fake_data = b"HTTP/1.1 200 OK\r\n".to_vec();
        println!("[{}:{}] ← {} bytes", self.host, self.port, fake_data.len());
        self.bytes_recv += fake_data.len();
        Ok((fake_data, self))
    }

    /// Transition: Connected → Closed
    pub fn close(self) -> TcpConn<Closed> {
        println!("Closing {}:{} (sent={}, recv={})",
            self.host, self.port, self.bytes_sent, self.bytes_recv);
        TcpConn {
            host: self.host,
            port: self.port,
            bytes_sent: self.bytes_sent,
            bytes_recv: self.bytes_recv,
            _state: PhantomData,
        }
    }

    pub fn peer(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl TcpConn<Closed> {
    pub fn bytes_sent(&self) -> usize { self.bytes_sent }
    pub fn bytes_recv(&self) -> usize { self.bytes_recv }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_then_close() {
        let conn = TcpConn::<Disconnected>::new("localhost", 8080);
        let conn = conn.connect().unwrap();
        let closed = conn.close();
        assert_eq!(closed.bytes_sent(), 0);
        assert_eq!(closed.bytes_recv(), 0);
    }

    #[test]
    fn send_recv_accumulates_bytes() {
        let conn = TcpConn::<Disconnected>::new("localhost", 8080)
            .connect().unwrap();
        let conn = conn.send(b"hello world").unwrap();
        let (_data, conn) = conn.recv().unwrap();
        let closed = conn.close();
        assert_eq!(closed.bytes_sent(), 11);
        assert!(closed.bytes_recv() > 0);
    }

    #[test]
    fn peer_returns_host_and_port() {
        let conn = TcpConn::<Disconnected>::new("example.com", 443)
            .connect().unwrap();
        assert_eq!(conn.peer(), "example.com:443");
        conn.close();
    }
}
