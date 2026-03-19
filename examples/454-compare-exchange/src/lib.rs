#![allow(clippy::manual_is_multiple_of)]
#![allow(unused_variables)]
#![allow(clippy::match_like_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::char_lit_as_u8)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::manual_strip)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure)]
#![allow(unused_imports)]
#![allow(dead_code)]
// 454. CAS: compare_exchange and loops
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn cas_increment(a: &AtomicUsize) {
    let mut cur = a.load(Ordering::Relaxed);
    loop {
        match a.compare_exchange_weak(cur, cur + 1, Ordering::AcqRel, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => cur = actual,
        }
    }
}

fn atomic_max(a: &AtomicI64, v: i64) {
    let mut cur = a.load(Ordering::Relaxed);
    loop {
        if v <= cur {
            break;
        }
        match a.compare_exchange_weak(cur, v, Ordering::AcqRel, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => cur = actual,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cas_inc() {
        let a = AtomicUsize::new(0);
        for _ in 0..100 {
            cas_increment(&a);
        }
        assert_eq!(a.load(Ordering::SeqCst), 100);
    }
    #[test]
    fn test_cas_fail() {
        let a = AtomicUsize::new(5);
        assert_eq!(
            a.compare_exchange(99, 0, Ordering::SeqCst, Ordering::SeqCst),
            Err(5)
        );
    }
    #[test]
    fn test_max() {
        let m = AtomicI64::new(i64::MIN);
        for v in [5, 3, 8, 1, 9, 2] {
            atomic_max(&m, v);
        }
        assert_eq!(m.load(Ordering::SeqCst), 9);
    }
}
