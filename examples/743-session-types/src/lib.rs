use std::collections::VecDeque;
/// 743: Session Types — protocol safety via typestate
/// The protocol: Connect → SendRequest → RecvResponse → Close
/// Violating the order is a COMPILE ERROR.
use std::marker::PhantomData;

// ── Protocol state markers ─────────────────────────────────────────────────────

pub struct Connected;
pub struct RequestSent;
pub struct ResponseReceived;
pub struct Closed;

// ── Channel (simulated in-memory) ─────────────────────────────────────────────

struct Channel {
    outbox: VecDeque<Vec<u8>>,
    inbox: VecDeque<Vec<u8>>,
}

impl Channel {
    fn new() -> Self {
        Channel {
            outbox: VecDeque::new(),
            inbox: VecDeque::new(),
        }
    }

    fn send(&mut self, data: Vec<u8>) {
        // Simulate: echo response into inbox
        let response = format!("RESP:{}", String::from_utf8_lossy(&data)).into_bytes();
        self.outbox.push_back(data);
        self.inbox.push_back(response);
    }

    fn recv(&mut self) -> Option<Vec<u8>> {
        self.inbox.pop_front()
    }
}

// ── Session ────────────────────────────────────────────────────────────────────

pub struct Session<State> {
    channel: Channel,
    log: Vec<String>,
    _state: PhantomData<State>,
}

/// Create a new session — starts in `Connected` state.
pub fn open_session() -> Session<Connected> {
    println!("[Session] Connected");
    Session {
        channel: Channel::new(),
        log: Vec::new(),
        _state: PhantomData,
    }
}

impl Session<Connected> {
    /// Connected → RequestSent (must happen before recv)
    pub fn send_request(mut self, method: &str, path: &str) -> Session<RequestSent> {
        let msg = format!("{} {}", method, path);
        println!("[Session] → Sending: {}", msg);
        self.channel.send(msg.into_bytes());
        self.log.push(format!("SENT: {} {}", method, path));
        Session {
            channel: self.channel,
            log: self.log,
            _state: PhantomData,
        }
    }
}

impl Session<RequestSent> {
    /// RequestSent → ResponseReceived
    pub fn receive_response(mut self) -> (String, Session<ResponseReceived>) {
        let data = self.channel.recv().expect("no response in channel");
        let response = String::from_utf8_lossy(&data).into_owned();
        println!("[Session] ← Received: {}", response);
        self.log.push(format!("RECV: {}", response));
        let sess = Session {
            channel: self.channel,
            log: self.log,
            _state: PhantomData,
        };
        (response, sess)
    }
}

impl Session<ResponseReceived> {
    /// ResponseReceived → Closed (or back to Connected for next request)
    pub fn close(mut self) -> Session<Closed> {
        println!("[Session] Closed. {} log entries.", self.log.len());
        Session {
            channel: self.channel,
            log: self.log,
            _state: PhantomData,
        }
    }

    /// Alternatively: send another request (pipeline)
    pub fn send_next_request(mut self, method: &str, path: &str) -> Session<RequestSent> {
        let msg = format!("{} {}", method, path);
        println!("[Session] → Pipeline: {}", msg);
        self.channel.send(msg.into_bytes());
        self.log.push(format!("SENT: {} {}", method, path));
        Session {
            channel: self.channel,
            log: self.log,
            _state: PhantomData,
        }
    }
}

impl Session<Closed> {
    pub fn log_entries(&self) -> &[String] {
        &self.log
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_compiles_and_runs() {
        let s = open_session();
        let s = s.send_request("GET", "/test");
        let (resp, s) = s.receive_response();
        let closed = s.close();
        assert!(resp.contains("GET /test"));
        assert_eq!(closed.log_entries().len(), 2);
    }

    #[test]
    fn response_echoes_request() {
        let s = open_session().send_request("POST", "/data");
        let (resp, _s) = s.receive_response();
        assert!(resp.contains("POST /data"), "got: {}", resp);
        _s.close();
    }

    #[test]
    fn pipeline_two_requests() {
        let s = open_session()
            .send_request("GET", "/a")
            .receive_response()
            .1
            .send_next_request("GET", "/b")
            .receive_response()
            .1
            .close();
        assert_eq!(s.log_entries().len(), 4);
    }
}
