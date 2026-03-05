// 455. Lock-free stack with atomics
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;
use std::ptr;

struct Node<T> { value: T, next: *mut Node<T> }
pub struct Stack<T> { head: AtomicPtr<Node<T>> }
unsafe impl<T:Send> Send for Stack<T> {}
unsafe impl<T:Send> Sync for Stack<T> {}

impl<T> Stack<T> {
    pub fn new() -> Self { Stack { head: AtomicPtr::new(ptr::null_mut()) } }
    pub fn push(&self, v: T) {
        let n = Box::into_raw(Box::new(Node { value:v, next:ptr::null_mut() }));
        loop {
            let h = self.head.load(Ordering::Relaxed);
            unsafe { (*n).next = h; }
            match self.head.compare_exchange_weak(h, n, Ordering::Release, Ordering::Relaxed) {
                Ok(_) => break, Err(_) => {}
            }
        }
    }
    pub fn pop(&self) -> Option<T> {
        loop {
            let h = self.head.load(Ordering::Acquire);
            if h.is_null() { return None; }
            let next = unsafe { (*h).next };
            match self.head.compare_exchange_weak(h, next, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(_) => { let v = unsafe { ptr::read(&(*h).value) }; unsafe { drop(Box::from_raw(h)); } return Some(v); }
                Err(_) => {}
            }
        }
    }
}
impl<T> Drop for Stack<T> { fn drop(&mut self) { while self.pop().is_some() {} } }


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_lifo()  { let s=Stack::new(); s.push(1); s.push(2); s.push(3); assert_eq!(s.pop(),Some(3)); assert_eq!(s.pop(),Some(2)); assert_eq!(s.pop(),None.or(Some(1))); assert_eq!(s.pop(),None); }
    #[test] fn test_concurrent() {
        let s=Arc::new(Stack::<u32>::new());
        let hs:Vec<_>=(0..4).map(|_|{let s=Arc::clone(&s); thread::spawn(move || { for i in 0..100 { s.push(i); } })}).collect();
        for h in hs { h.join().unwrap(); }
        let mut c=0; while s.pop().is_some() { c+=1; } assert_eq!(c,400);
    }
}
