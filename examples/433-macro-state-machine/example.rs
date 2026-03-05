// State machine via macro in Rust — typestate pattern

// Macro that generates a typestate state machine
macro_rules! state_machine {
    (
        struct $name:ident<$state_param:ident> {
            $($field:ident : $fty:ty),* $(,)?
        }
        states { $($state:ident),* $(,)? }
        transitions {
            $( $from:ident => $method:ident => $to:ident { $($body:tt)* } )*
        }
    ) => {
        // State marker types
        $(
            #[derive(Debug)]
            struct $state;
        )*

        // The state machine struct
        #[derive(Debug)]
        struct $name<S> {
            $($field: $fty,)*
            _state: std::marker::PhantomData<S>,
        }

        // Transition impls
        $(
            impl $name<$from> {
                fn $method(self) -> $name<$to> {
                    $name {
                        $($field: self.$field,)*
                        _state: std::marker::PhantomData,
                    }
                }
            }
        )*
    };
}

// Define a Connection state machine
#[derive(Debug)]
struct Disconnected;
#[derive(Debug)]
struct Connected;
#[derive(Debug)]
struct Authenticated;
#[derive(Debug)]
struct Closed;

#[derive(Debug)]
struct Connection<State> {
    host: String,
    port: u16,
    messages_sent: u32,
    _state: std::marker::PhantomData<State>,
}

impl Connection<Disconnected> {
    fn new(host: &str, port: u16) -> Self {
        Connection {
            host: host.to_string(),
            port,
            messages_sent: 0,
            _state: std::marker::PhantomData,
        }
    }

    fn connect(self) -> Connection<Connected> {
        println!("Connecting to {}:{}", self.host, self.port);
        Connection { _state: std::marker::PhantomData, ..self }
    }
}

impl Connection<Connected> {
    fn authenticate(self, _token: &str) -> Connection<Authenticated> {
        println!("Authenticating...");
        Connection { _state: std::marker::PhantomData, ..self }
    }

    fn disconnect(self) -> Connection<Closed> {
        println!("Disconnecting (unauthenticated)");
        Connection { _state: std::marker::PhantomData, ..self }
    }
}

impl Connection<Authenticated> {
    fn send(&mut self, message: &str) {
        println!("Sending: {}", message);
        self.messages_sent += 1;
    }

    fn disconnect(self) -> Connection<Closed> {
        println!("Disconnecting (sent {} messages)", self.messages_sent);
        Connection { _state: std::marker::PhantomData, ..self }
    }
}

impl Connection<Closed> {
    fn stats(&self) {
        println!("Connection to {} closed. Messages sent: {}",
                 self.host, self.messages_sent);
    }
}

fn main() {
    let conn = Connection::new("api.example.com", 443);
    let conn = conn.connect();
    let mut conn = conn.authenticate("secret-token");

    conn.send("GET /users HTTP/1.1");
    conn.send("Host: api.example.com");

    let closed = conn.disconnect();
    closed.stats();

    // Type safety: these would NOT compile:
    // conn.send("too late!"); // Connection<Closed> has no send()
    // let unauthenticated = Connection::new("h", 80).connect();
    // unauthenticated.send("no auth!"); // Connected has no send()

    println!("
State machine enforced at compile time!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine() {
        let conn = Connection::new("test.com", 80)
            .connect()
            .authenticate("token");
        let closed = conn.disconnect();
        // Can only call stats() on Closed
        closed.stats();
    }

    fn assert_send_type<T: Send>() {}

    #[test]
    fn test_connection_types_distinct() {
        // These are different types at compile time:
        let _disc: Connection<Disconnected> = Connection::new("h", 80);
        let _conn: Connection<Connected> = Connection::new("h", 80).connect();
        // They don't have the same methods — enforced by type system
    }
}
