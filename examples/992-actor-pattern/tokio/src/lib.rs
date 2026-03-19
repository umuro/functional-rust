#![allow(clippy::all)]
// 992: Actor Pattern — Tokio version
// Actors using tokio::sync::mpsc channels and spawned tasks

use tokio::sync::{mpsc, oneshot};

// --- Counter Actor ---
enum CounterMsg {
    Increment(i64),
    Decrement(i64),
    GetValue(oneshot::Sender<i64>),
    Shutdown,
}

struct CounterActor {
    tx: mpsc::Sender<CounterMsg>,
}

impl CounterActor {
    fn spawn() -> Self {
        let (tx, mut rx) = mpsc::channel::<CounterMsg>(32);
        tokio::spawn(async move {
            let mut state: i64 = 0;
            while let Some(msg) = rx.recv().await {
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

    async fn increment(&self, n: i64) {
        self.tx.send(CounterMsg::Increment(n)).await.unwrap();
    }

    async fn decrement(&self, n: i64) {
        self.tx.send(CounterMsg::Decrement(n)).await.unwrap();
    }

    async fn get_value(&self) -> i64 {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx.send(CounterMsg::GetValue(reply_tx)).await.unwrap();
        reply_rx.await.unwrap()
    }

    async fn shutdown(self) {
        self.tx.send(CounterMsg::Shutdown).await.ok();
    }
}

// --- Adder Actor with request-response via oneshot ---
enum AdderMsg {
    Add { a: i32, b: i32, reply: oneshot::Sender<i32> },
    Stop,
}

struct AdderActor {
    tx: mpsc::Sender<AdderMsg>,
}

impl AdderActor {
    fn spawn() -> Self {
        let (tx, mut rx) = mpsc::channel::<AdderMsg>(32);
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    AdderMsg::Add { a, b, reply } => { reply.send(a + b).ok(); }
                    AdderMsg::Stop => break,
                }
            }
        });
        AdderActor { tx }
    }

    async fn add(&self, a: i32, b: i32) -> i32 {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx.send(AdderMsg::Add { a, b, reply: reply_tx }).await.unwrap();
        reply_rx.await.unwrap()
    }

    async fn stop(self) { self.tx.send(AdderMsg::Stop).await.ok(); }
}

// --- Traffic Light State Machine Actor ---
#[derive(Debug, PartialEq, Clone)]
enum TrafficLight { Red, Yellow, Green }

enum TrafficMsg {
    Next,
    GetState(oneshot::Sender<TrafficLight>),
    Stop,
}

struct TrafficActor { tx: mpsc::Sender<TrafficMsg> }

impl TrafficActor {
    fn spawn() -> Self {
        let (tx, mut rx) = mpsc::channel::<TrafficMsg>(32);
        tokio::spawn(async move {
            let mut state = TrafficLight::Red;
            while let Some(msg) = rx.recv().await {
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

    async fn next(&self) { self.tx.send(TrafficMsg::Next).await.unwrap(); }

    async fn state(&self) -> TrafficLight {
        let (r, rx) = oneshot::channel();
        self.tx.send(TrafficMsg::GetState(r)).await.unwrap();
        rx.await.unwrap()
    }

    async fn stop(self) { self.tx.send(TrafficMsg::Stop).await.ok(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_counter_actor() {
        let actor = CounterActor::spawn();
        actor.increment(10).await;
        actor.increment(5).await;
        actor.decrement(3).await;
        assert_eq!(actor.get_value().await, 12);
        actor.shutdown().await;
    }

    #[tokio::test]
    async fn test_adder_actor() {
        let adder = AdderActor::spawn();
        assert_eq!(adder.add(17, 25).await, 42);
        assert_eq!(adder.add(1, 1).await, 2);
        adder.stop().await;
    }

    #[tokio::test]
    async fn test_traffic_light_actor() {
        let t = TrafficActor::spawn();
        assert_eq!(t.state().await, TrafficLight::Red);
        t.next().await;
        assert_eq!(t.state().await, TrafficLight::Green);
        t.next().await;
        assert_eq!(t.state().await, TrafficLight::Yellow);
        t.next().await;
        assert_eq!(t.state().await, TrafficLight::Red);
        t.stop().await;
    }

    #[tokio::test]
    async fn test_counter_negative() {
        let actor = CounterActor::spawn();
        actor.decrement(5).await;
        assert_eq!(actor.get_value().await, -5);
        actor.shutdown().await;
    }
}
