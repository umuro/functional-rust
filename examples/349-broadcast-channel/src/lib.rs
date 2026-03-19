//! # Broadcast Channel
//! One sender, many receivers — every subscriber gets a copy of every message.

use std::sync::{mpsc, Arc, Mutex};

pub struct BroadcastSender<T: Clone + Send + 'static> {
    subscribers: Arc<Mutex<Vec<mpsc::SyncSender<T>>>>,
}

pub struct BroadcastReceiver<T> {
    rx: mpsc::Receiver<T>,
}

impl<T: Clone + Send + 'static> BroadcastSender<T> {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn subscribe(&self, buf: usize) -> BroadcastReceiver<T> {
        let (tx, rx) = mpsc::sync_channel(buf);
        self.subscribers.lock().unwrap().push(tx);
        BroadcastReceiver { rx }
    }

    pub fn send(&self, msg: T) {
        let subs = self.subscribers.lock().unwrap();
        for sub in subs.iter() {
            let _ = sub.try_send(msg.clone());
        }
    }
}

impl<T: Clone + Send + 'static> Default for BroadcastSender<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BroadcastReceiver<T> {
    pub fn recv(&self) -> Option<T> {
        self.rx.recv().ok()
    }
    pub fn try_recv(&self) -> Option<T> {
        self.rx.try_recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn broadcast_to_multiple() {
        let sender = BroadcastSender::new();
        let r1 = sender.subscribe(10);
        let r2 = sender.subscribe(10);
        sender.send(42);
        assert_eq!(r1.recv(), Some(42));
        assert_eq!(r2.recv(), Some(42));
    }
    #[test]
    fn late_subscriber_misses() {
        let sender = BroadcastSender::new();
        sender.send(1);
        let r = sender.subscribe(10);
        sender.send(2);
        assert_eq!(r.recv(), Some(2));
    }
}
