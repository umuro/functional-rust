//! # Lock-Free Queue — MPSC Queue Pattern
//!
//! A lock-free MPSC (multiple producer, single consumer) queue.

use std::cell::UnsafeCell;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

/// Node in the queue
struct Node<T> {
    data: Option<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(data: Option<T>) -> *mut Self {
        Box::into_raw(Box::new(Self {
            data,
            next: AtomicPtr::new(ptr::null_mut()),
        }))
    }
}

/// Simple MPSC lock-free queue
/// Multiple producers can enqueue, single consumer dequeues
pub struct MpscQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: UnsafeCell<*mut Node<T>>,
}

unsafe impl<T: Send> Send for MpscQueue<T> {}
unsafe impl<T: Send> Sync for MpscQueue<T> {}

impl<T> MpscQueue<T> {
    pub fn new() -> Self {
        let stub = Node::new(None);
        Self {
            head: AtomicPtr::new(stub),
            tail: UnsafeCell::new(stub),
        }
    }

    /// Push (can be called from multiple threads)
    pub fn push(&self, data: T) {
        let node = Node::new(Some(data));

        // Atomically swap the head to our new node
        let prev = self.head.swap(node, Ordering::AcqRel);

        // Link the previous head to our new node
        unsafe {
            (*prev).next.store(node, Ordering::Release);
        }
    }

    /// Pop (single consumer only!)
    pub fn pop(&self) -> Option<T> {
        unsafe {
            let tail = *self.tail.get();
            let next = (*tail).next.load(Ordering::Acquire);

            if next.is_null() {
                return None;
            }

            *self.tail.get() = next;
            let data = (*next).data.take();

            // Free old tail
            let _ = Box::from_raw(tail);

            data
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            let tail = *self.tail.get();
            (*tail).next.load(Ordering::Acquire).is_null()
        }
    }
}

impl<T> Default for MpscQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for MpscQueue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
        // Free the stub node
        unsafe {
            let _ = Box::from_raw(*self.tail.get());
        }
    }
}

/// Safe bounded queue using channels
pub mod bounded {
    use std::sync::mpsc::{self, Receiver, SyncSender};

    pub struct BoundedQueue<T> {
        sender: SyncSender<T>,
        receiver: Receiver<T>,
    }

    impl<T> BoundedQueue<T> {
        pub fn new(capacity: usize) -> Self {
            let (sender, receiver) = mpsc::sync_channel(capacity);
            Self { sender, receiver }
        }

        pub fn push(&self, value: T) -> Result<(), T> {
            self.sender.send(value).map_err(|e| e.0)
        }

        pub fn try_push(&self, value: T) -> Result<(), T> {
            self.sender.try_send(value).map_err(|e| match e {
                mpsc::TrySendError::Full(v) => v,
                mpsc::TrySendError::Disconnected(v) => v,
            })
        }

        pub fn pop(&self) -> Option<T> {
            self.receiver.recv().ok()
        }

        pub fn try_pop(&self) -> Option<T> {
            self.receiver.try_recv().ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_mpsc_single_thread() {
        let queue = MpscQueue::new();

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_mpsc_is_empty() {
        let queue = MpscQueue::new();
        assert!(queue.is_empty());

        queue.push(1);
        assert!(!queue.is_empty());

        queue.pop();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_mpsc_multiple_producers() {
        let queue = Arc::new(MpscQueue::new());

        let producers: Vec<_> = (0..4)
            .map(|i| {
                let q = Arc::clone(&queue);
                thread::spawn(move || {
                    for j in 0..100 {
                        q.push(i * 100 + j);
                    }
                })
            })
            .collect();

        for p in producers {
            p.join().unwrap();
        }

        let mut count = 0;
        while queue.pop().is_some() {
            count += 1;
        }
        assert_eq!(count, 400);
    }

    #[test]
    fn test_bounded_queue() {
        let queue = bounded::BoundedQueue::new(2);

        assert!(queue.push(1).is_ok());
        assert!(queue.push(2).is_ok());
        assert!(queue.try_push(3).is_err()); // Full

        assert_eq!(queue.pop(), Some(1));
        assert!(queue.try_push(3).is_ok()); // Space available

        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
    }

    #[test]
    fn test_fifo_order() {
        let queue = MpscQueue::new();

        for i in 0..10 {
            queue.push(i);
        }

        for i in 0..10 {
            assert_eq!(queue.pop(), Some(i));
        }
    }
}
