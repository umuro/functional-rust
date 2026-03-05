//! # Lock-Free Stack — Compare-and-Swap Based Data Structure
//!
//! A thread-safe stack using atomic operations instead of locks.

use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

/// A node in the lock-free stack
struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

/// Lock-free stack using CAS (compare-and-swap)
pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Push a value onto the stack
    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Relaxed);
            unsafe {
                (*new_node).next = head;
            }

            // CAS: if head is still the same, update it to new_node
            if self
                .head
                .compare_exchange_weak(head, new_node, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
            // If CAS failed, retry with updated head
        }
    }

    /// Pop a value from the stack
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);

            if head.is_null() {
                return None;
            }

            let next = unsafe { (*head).next };

            // CAS: if head is still the same, update it to next
            if self
                .head
                .compare_exchange_weak(head, next, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                // Successfully popped
                let data = unsafe { Box::from_raw(head).data };
                return Some(data);
            }
            // If CAS failed, retry
        }
    }

    /// Check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Default for LockFreeStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

// Note: This implementation is NOT fully safe for concurrent use
// due to the ABA problem. A production implementation would need
// hazard pointers or epoch-based reclamation.

/// Safe wrapper for demonstration
pub mod safe {
    use std::sync::Mutex;

    pub struct Stack<T> {
        inner: Mutex<Vec<T>>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self {
                inner: Mutex::new(Vec::new()),
            }
        }

        pub fn push(&self, value: T) {
            self.inner.lock().unwrap().push(value);
        }

        pub fn pop(&self) -> Option<T> {
            self.inner.lock().unwrap().pop()
        }

        pub fn is_empty(&self) -> bool {
            self.inner.lock().unwrap().is_empty()
        }
    }

    impl<T> Default for Stack<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_push_pop_single_thread() {
        let stack = LockFreeStack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_is_empty() {
        let stack = LockFreeStack::new();
        assert!(stack.is_empty());

        stack.push(1);
        assert!(!stack.is_empty());

        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_safe_stack_concurrent() {
        let stack = Arc::new(safe::Stack::new());

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let s = Arc::clone(&stack);
                thread::spawn(move || {
                    for j in 0..100 {
                        s.push(i * 100 + j);
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        let mut count = 0;
        while stack.pop().is_some() {
            count += 1;
        }
        assert_eq!(count, 400);
    }

    #[test]
    fn test_compare_exchange() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let value = AtomicUsize::new(5);

        // Successful CAS
        let result = value.compare_exchange(5, 10, Ordering::SeqCst, Ordering::SeqCst);
        assert_eq!(result, Ok(5));
        assert_eq!(value.load(Ordering::SeqCst), 10);

        // Failed CAS (expected != current)
        let result = value.compare_exchange(5, 20, Ordering::SeqCst, Ordering::SeqCst);
        assert_eq!(result, Err(10));
    }

    #[test]
    fn test_compare_exchange_weak() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let value = AtomicUsize::new(0);
        let mut attempts = 0;

        // compare_exchange_weak may spuriously fail
        while value
            .compare_exchange_weak(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            attempts += 1;
            if attempts > 100 {
                panic!("Too many spurious failures");
            }
        }

        assert_eq!(value.load(Ordering::SeqCst), 1);
    }
}
