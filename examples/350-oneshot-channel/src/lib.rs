//! # Oneshot Channel
//! Send exactly one value from one task to another.

use std::sync::{Arc, Condvar, Mutex};

pub struct OneshotSender<T> {
    state: Arc<(Mutex<Option<T>>, Condvar)>,
}
pub struct OneshotReceiver<T> {
    state: Arc<(Mutex<Option<T>>, Condvar)>,
}

pub fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let state = Arc::new((Mutex::new(None), Condvar::new()));
    (
        OneshotSender {
            state: Arc::clone(&state),
        },
        OneshotReceiver { state },
    )
}

impl<T> OneshotSender<T> {
    pub fn send(self, value: T) {
        let (lock, cvar) = &*self.state;
        *lock.lock().unwrap() = Some(value);
        cvar.notify_one();
    }
}

impl<T> OneshotReceiver<T> {
    pub fn recv(self) -> T {
        let (lock, cvar) = &*self.state;
        let mut guard = lock.lock().unwrap();
        while guard.is_none() {
            guard = cvar.wait(guard).unwrap();
        }
        guard.take().unwrap()
    }
    pub fn try_recv(&self) -> Option<T> {
        self.state.0.lock().unwrap().take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn oneshot_sends_value() {
        let (tx, rx) = oneshot();
        tx.send(42);
        assert_eq!(rx.recv(), 42);
    }
    #[test]
    fn oneshot_across_threads() {
        let (tx, rx) = oneshot();
        thread::spawn(move || tx.send("hello"));
        assert_eq!(rx.recv(), "hello");
    }
    #[test]
    fn try_recv_before() {
        let (_tx, rx) = oneshot::<i32>();
        assert!(rx.try_recv().is_none());
    }
}
