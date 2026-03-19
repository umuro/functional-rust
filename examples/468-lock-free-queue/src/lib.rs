// 468. Lock-free queue basics (Michael-Scott, simplified)
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}
impl<T> Node<T> {
    fn new(v: Option<T>) -> *mut Self {
        Box::into_raw(Box::new(Node {
            value: v,
            next: AtomicPtr::new(ptr::null_mut()),
        }))
    }
}

pub struct Queue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}
unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Send> Sync for Queue<T> {}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let d = Node::new(None);
        Queue {
            head: AtomicPtr::new(d),
            tail: AtomicPtr::new(d),
        }
    }
    pub fn enqueue(&self, v: T) {
        let n = Node::new(Some(v));
        loop {
            let t = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*t).next.load(Ordering::Acquire) };
            if next.is_null() {
                match unsafe {
                    (*t).next.compare_exchange_weak(
                        ptr::null_mut(),
                        n,
                        Ordering::Release,
                        Ordering::Relaxed,
                    )
                } {
                    Ok(_) => {
                        let _ =
                            self.tail
                                .compare_exchange(t, n, Ordering::Release, Ordering::Relaxed);
                        return;
                    }
                    Err(_) => {}
                }
            } else {
                let _ = self
                    .tail
                    .compare_exchange(t, next, Ordering::Release, Ordering::Relaxed);
            }
        }
    }
    pub fn dequeue(&self) -> Option<T> {
        loop {
            let h = self.head.load(Ordering::Acquire);
            let t = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*h).next.load(Ordering::Acquire) };
            if h == t {
                if next.is_null() {
                    return None;
                }
                let _ = self
                    .tail
                    .compare_exchange(t, next, Ordering::Release, Ordering::Relaxed);
            } else {
                match self
                    .head
                    .compare_exchange_weak(h, next, Ordering::AcqRel, Ordering::Relaxed)
                {
                    Ok(_) => {
                        let v = unsafe { ptr::read(&(*next).value) };
                        unsafe { drop(Box::from_raw(h)) };
                        return v;
                    }
                    Err(_) => {}
                }
            }
        }
    }
}
impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
        unsafe {
            drop(Box::from_raw(self.head.load(Ordering::Relaxed)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fifo() {
        let q = Queue::new();
        for i in 1..=5u32 {
            q.enqueue(i);
        }
        for i in 1..=5 {
            assert_eq!(q.dequeue(), Some(i));
        }
        assert_eq!(q.dequeue(), None);
    }
    #[test]
    fn test_concurrent() {
        let q = Arc::new(Queue::<u32>::new());
        thread::scope(|s| {
            for i in 0..4u32 {
                let q = Arc::clone(&q);
                s.spawn(move || {
                    for j in 0..25 {
                        q.enqueue(i * 25 + j);
                    }
                });
            }
        });
        let mut c = 0;
        while q.dequeue().is_some() {
            c += 1;
        }
        assert_eq!(c, 100);
    }
}
