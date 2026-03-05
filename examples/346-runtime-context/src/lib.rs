//! # Runtime Context
//! Task-local storage and context propagation in async code.

use std::cell::RefCell;
use std::thread_local;

thread_local! {
    static CONTEXT: RefCell<Option<String>> = RefCell::new(None);
}

pub fn set_context(ctx: String) { CONTEXT.with(|c| *c.borrow_mut() = Some(ctx)); }
pub fn get_context() -> Option<String> { CONTEXT.with(|c| c.borrow().clone()) }
pub fn clear_context() { CONTEXT.with(|c| *c.borrow_mut() = None); }

pub fn with_context<R>(ctx: String, f: impl FnOnce() -> R) -> R {
    let old = get_context();
    set_context(ctx);
    let result = f();
    match old { Some(c) => set_context(c), None => clear_context() }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn context_propagation() {
        set_context("test".into());
        assert_eq!(get_context(), Some("test".into()));
        clear_context();
        assert_eq!(get_context(), None);
    }
    #[test] fn with_context_restores() {
        set_context("outer".into());
        let inner = with_context("inner".into(), || get_context());
        assert_eq!(inner, Some("inner".into()));
        assert_eq!(get_context(), Some("outer".into()));
    }
}
