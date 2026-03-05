// 992: Actor Pattern
// Rust: enum Message + thread + mpsc channel mailbox

use std::sync::mpsc;
use std::thread;

// --- Approach 1: Counter actor ---
#[derive(Debug)]
enum CounterMsg {
    Increment(i64),
    Decrement(i64),
    GetValue(mpsc::Sender<i64>),
    Shutdown,
}

struct CounterActor {
    tx: mpsc::Sender<CounterMsg>,
}

impl CounterActor {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel::<CounterMsg>();
        thread::spawn(move || {
            let mut state: i64 = 0;
            for msg in rx.iter() {
                match msg {
                    CounterMsg::Increment(n) => state += n,
                    CounterMsg::Decrement(n) => state -= n,
                    CounterMsg::GetValue(reply) => { reply.send(state).ok(); }
                    CounterMsg::Shutdown => break,
                }
            }
        });
        CounterActor { tx }
    }

    fn increment(&self, n: i64) { self.tx.send(CounterMsg::Increment(n)).unwrap(); }
    fn decrement(&self, n: i64) { self.tx.send(CounterMsg::Decrement(n)).unwrap(); }

    fn get_value(&self) -> i64 {
        let (reply_tx, reply_rx) = mpsc::channel();
        self.tx.send(CounterMsg::GetValue(reply_tx)).unwrap();
        reply_rx.recv().unwrap()
    }

    fn shutdown(self) { self.tx.send(CounterMsg::Shutdown).ok(); }
}

// --- Approach 2: Generic actor with request-response ---
#[derive(Debug)]
enum AdderMsg {
    Add { a: i32, b: i32, reply: mpsc::Sender<i32> },
    Stop,
}

struct AdderActor {
    tx: mpsc::Sender<AdderMsg>,
}

impl AdderActor {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel::<AdderMsg>();
        thread::spawn(move || {
            for msg in rx.iter() {
                match msg {
                    AdderMsg::Add { a, b, reply } => { reply.send(a + b).ok(); }
                    AdderMsg::Stop => break,
                }
            }
        });
        AdderActor { tx }
    }

    fn add(&self, a: i32, b: i32) -> i32 {
        let (reply_tx, reply_rx) = mpsc::channel();
        self.tx.send(AdderMsg::Add { a, b, reply: reply_tx }).unwrap();
        reply_rx.recv().unwrap()
    }

    fn stop(self) { self.tx.send(AdderMsg::Stop).ok(); }
}

// --- Approach 3: State machine actor ---
#[derive(Debug, PartialEq, Clone)]
enum TrafficLight { Red, Yellow, Green }

#[derive(Debug)]
enum TrafficMsg {
    Next,
    GetState(mpsc::Sender<TrafficLight>),
    Stop,
}

struct TrafficActor { tx: mpsc::Sender<TrafficMsg> }

impl TrafficActor {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel::<TrafficMsg>();
        thread::spawn(move || {
            let mut state = TrafficLight::Red;
            for msg in rx.iter() {
                match msg {
                    TrafficMsg::Next => {
                        state = match state {
                            TrafficLight::Red => TrafficLight::Green,
                            TrafficLight::Green => TrafficLight::Yellow,
                            TrafficLight::Yellow => TrafficLight::Red,
                        };
                    }
                    TrafficMsg::GetState(reply) => { reply.send(state.clone()).ok(); }
                    TrafficMsg::Stop => break,
                }
            }
        });
        TrafficActor { tx }
    }

    fn next(&self) { self.tx.send(TrafficMsg::Next).unwrap(); }

    fn state(&self) -> TrafficLight {
        let (r, rx) = mpsc::channel();
        self.tx.send(TrafficMsg::GetState(r)).unwrap();
        rx.recv().unwrap()
    }

    fn stop(self) { self.tx.send(TrafficMsg::Stop).ok(); }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_actor() {
        let actor = CounterActor::spawn();
        actor.increment(10);
        actor.increment(5);
        actor.decrement(3);
        assert_eq!(actor.get_value(), 12);
        actor.shutdown();
    }

    #[test]
    fn test_adder_actor() {
        let adder = AdderActor::spawn();
        assert_eq!(adder.add(17, 25), 42);
        assert_eq!(adder.add(1, 1), 2);
        adder.stop();
    }

    #[test]
    fn test_traffic_light_actor() {
        let t = TrafficActor::spawn();
        assert_eq!(t.state(), TrafficLight::Red);
        t.next();
        assert_eq!(t.state(), TrafficLight::Green);
        t.next();
        assert_eq!(t.state(), TrafficLight::Yellow);
        t.next();
        assert_eq!(t.state(), TrafficLight::Red);
        t.stop();
    }

    #[test]
    fn test_counter_negative() {
        let actor = CounterActor::spawn();
        actor.decrement(5);
        assert_eq!(actor.get_value(), -5);
        actor.shutdown();
    }
}
