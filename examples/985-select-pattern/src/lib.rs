// 985: Select Pattern — Poll Multiple Channels
// Rust: try_recv loop for non-blocking select over multiple channels

use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq)]
enum Selected<A, B> {
    Left(A),
    Right(B),
    BothClosed,
}

// --- Non-blocking select over two channels ---
fn select<A, B>(
    rx1: &mpsc::Receiver<A>,
    rx2: &mpsc::Receiver<B>,
) -> Selected<A, B> {
    let mut r1_closed = false;
    let mut r2_closed = false;
    loop {
        if !r1_closed {
            match rx1.try_recv() {
                Ok(v) => return Selected::Left(v),
                Err(TryRecvError::Disconnected) => r1_closed = true,
                Err(TryRecvError::Empty) => {}
            }
        }
        if !r2_closed {
            match rx2.try_recv() {
                Ok(v) => return Selected::Right(v),
                Err(TryRecvError::Disconnected) => r2_closed = true,
                Err(TryRecvError::Empty) => {}
            }
        }
        if r1_closed && r2_closed {
            return Selected::BothClosed;
        }
        thread::yield_now();
    }
}

// --- Drain both channels via select, categorizing messages ---
fn select_drain(rx1: mpsc::Receiver<i32>, rx2: mpsc::Receiver<String>) -> (Vec<i32>, Vec<String>) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();

    loop {
        match select(&rx1, &rx2) {
            Selected::Left(v) => lefts.push(v),
            Selected::Right(v) => rights.push(v),
            Selected::BothClosed => break,
        }
    }
    (lefts, rights)
}

// --- Priority select: prefer channel 1 when both have data ---
fn priority_recv<T>(high: &mpsc::Receiver<T>, low: &mpsc::Receiver<T>) -> Option<(T, bool)> {
    // true = came from high priority
    match high.try_recv() {
        Ok(v) => Some((v, true)),
        Err(_) => low.try_recv().ok().map(|v| (v, false)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_drain() {
        let (tx1, rx1) = mpsc::channel::<i32>();
        let (tx2, rx2) = mpsc::channel::<String>();

        for i in [1, 2, 3] { tx1.send(i).unwrap(); }
        for s in ["a", "b", "c"] { tx2.send(s.to_string()).unwrap(); }
        drop(tx1); drop(tx2);

        let (mut lefts, mut rights) = select_drain(rx1, rx2);
        lefts.sort();
        rights.sort();
        assert_eq!(lefts, vec![1, 2, 3]);
        assert_eq!(rights, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_both_closed() {
        let (tx1, rx1) = mpsc::channel::<i32>();
        let (tx2, rx2) = mpsc::channel::<i32>();
        drop(tx1); drop(tx2);
        assert_eq!(select(&rx1, &rx2), Selected::BothClosed);
    }

    #[test]
    fn test_priority_recv() {
        let (htx, hrx) = mpsc::channel::<i32>();
        let (ltx, lrx) = mpsc::channel::<i32>();

        htx.send(10).unwrap();
        ltx.send(20).unwrap();

        // High priority wins
        let result = priority_recv(&hrx, &lrx);
        assert_eq!(result, Some((10, true)));

        // Now only low available
        let result2 = priority_recv(&hrx, &lrx);
        assert_eq!(result2, Some((20, false)));
    }

    #[test]
    fn test_select_empty_left() {
        let (_tx1, rx1) = mpsc::channel::<i32>();
        let (tx2, rx2) = mpsc::channel::<i32>();
        tx2.send(99).unwrap();
        drop(tx2);
        // rx1 never closes so we'll get Right(99) first
        assert_eq!(select(&rx1, &rx2), Selected::Right(99));
    }
}
