#![allow(clippy::all)]
// 464. Actor model in Rust
use std::sync::mpsc;
use std::thread;

enum Msg {
    Inc(i64),
    Dec(i64),
    Get(mpsc::SyncSender<i64>),
    Reset,
    Stop,
}

struct Actor {
    tx: mpsc::Sender<Msg>,
}

impl Actor {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Msg>();
        thread::spawn(move || {
            let mut s = 0i64;
            for m in rx {
                match m {
                    Msg::Inc(n) => s += n,
                    Msg::Dec(n) => s -= n,
                    Msg::Get(tx) => {
                        let _ = tx.send(s);
                    }
                    Msg::Reset => s = 0,
                    Msg::Stop => break,
                }
            }
        });
        Actor { tx }
    }
    fn inc(&self, n: i64) {
        self.tx.send(Msg::Inc(n)).unwrap();
    }
    fn dec(&self, n: i64) {
        self.tx.send(Msg::Dec(n)).unwrap();
    }
    fn reset(&self) {
        self.tx.send(Msg::Reset).unwrap();
    }
    fn get(&self) -> i64 {
        let (t, r) = mpsc::sync_channel(1);
        self.tx.send(Msg::Get(t)).unwrap();
        r.recv().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_actor() {
        let a = Actor::new();
        a.inc(7);
        a.inc(3);
        assert_eq!(a.get(), 10);
        a.dec(4);
        assert_eq!(a.get(), 6);
        a.reset();
        assert_eq!(a.get(), 0);
    }
}
