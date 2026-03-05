// 454. CAS: compare_exchange and loops
use std::sync::atomic::{AtomicUsize, AtomicI64, Ordering};
use std::sync::Arc;
use std::thread;

fn cas_increment(a: &AtomicUsize) {
    let mut cur = a.load(Ordering::Relaxed);
    loop {
        match a.compare_exchange_weak(cur, cur+1, Ordering::AcqRel, Ordering::Relaxed) {
            Ok(_)      => break,
            Err(actual) => cur = actual,
        }
    }
}

fn atomic_max(a: &AtomicI64, v: i64) {
    let mut cur = a.load(Ordering::Relaxed);
    loop {
        if v <= cur { break; }
        match a.compare_exchange_weak(cur, v, Ordering::AcqRel, Ordering::Relaxed) {
            Ok(_)        => break,
            Err(actual)  => cur = actual,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_cas_inc()  { let a=AtomicUsize::new(0); for _ in 0..100 { cas_increment(&a); } assert_eq!(a.load(Ordering::SeqCst),100); }
    #[test] fn test_cas_fail() { let a=AtomicUsize::new(5); assert_eq!(a.compare_exchange(99,0,Ordering::SeqCst,Ordering::SeqCst), Err(5)); }
    #[test] fn test_max()      { let m=AtomicI64::new(i64::MIN); for v in [5,3,8,1,9,2] { atomic_max(&m,v); } assert_eq!(m.load(Ordering::SeqCst),9); }
}
