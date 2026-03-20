#![allow(clippy::all)]
// 489. Arc<str> for shared strings
use std::sync::Arc;
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_arc_str_send() {
        let s: Arc<str> = Arc::from("hello");
        let s2 = Arc::clone(&s);
        thread::spawn(move || assert_eq!(&*s2, "hello"))
            .join()
            .unwrap();
    }
    #[test]
    fn test_ptr_eq() {
        let s: Arc<str> = Arc::from("hi");
        let c = Arc::clone(&s);
        assert!(Arc::ptr_eq(&s, &c));
    }
    #[test]
    fn test_deref() {
        let s: Arc<str> = Arc::from("hi");
        assert_eq!(s.len(), 2);
    }
}
