#![allow(clippy::all)]
//! Auto Traits: Send, Sync, Unpin

use std::sync::Arc;

pub fn is_send<T: Send>() {}
pub fn is_sync<T: Sync>() {}
pub fn is_unpin<T: Unpin>() {}

pub fn check_auto_traits() {
    is_send::<i32>();
    is_sync::<i32>();
    is_send::<String>();
    is_sync::<String>();
    is_send::<Arc<i32>>();
    is_sync::<Arc<i32>>();
    // is_send::<Rc<i32>>(); // Would fail - Rc is !Send
    // is_sync::<RefCell<i32>>(); // Would fail - RefCell is !Sync
}

pub struct MySendSync {
    pub data: Arc<String>,
}
unsafe impl Send for MySendSync {}
unsafe impl Sync for MySendSync {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_send() {
        fn assert_send<T: Send>() {}
        assert_send::<i32>();
    }
    #[test]
    fn test_i32_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<i32>();
    }
    #[test]
    fn test_arc_send_sync() {
        fn assert_both<T: Send + Sync>() {}
        assert_both::<Arc<i32>>();
    }
    #[test]
    fn test_string_send() {
        fn assert_send<T: Send>() {}
        assert_send::<String>();
    }
}
