// 467. Epoch-based garbage collection concept
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

struct EpochMgr {
    epoch: AtomicU64,
    retired: Mutex<VecDeque<(u64, String)>>,
    pinned: Mutex<Vec<u64>>,
}

impl EpochMgr {
    fn new() -> Self {
        EpochMgr { epoch: AtomicU64::new(0), retired: Mutex::new(VecDeque::new()), pinned: Mutex::new(Vec::new()) }
    }
    fn pin(&self) -> u64 {
        let e = self.epoch.load(Ordering::Acquire);
        self.pinned.lock().unwrap().push(e); e
    }
    fn unpin(&self, e: u64) {
        let mut p = self.pinned.lock().unwrap();
        if let Some(i) = p.iter().position(|&x| x==e) { p.remove(i); }
    }
    fn retire(&self, desc: &str) {
        let e = self.epoch.load(Ordering::Relaxed);
        self.retired.lock().unwrap().push_back((e, desc.to_string()));
    }
    fn collect(&self) {
        let new_e = self.epoch.fetch_add(1, Ordering::AcqRel) + 1;
        let min_active = self.pinned.lock().unwrap().iter().cloned().min().unwrap_or(new_e);
        let safe_before = min_active.saturating_sub(1);
        let mut r = self.retired.lock().unwrap(); let mut n=0;
        while r.front().map(|(e,_)| *e<=safe_before).unwrap_or(false) {
            let (_,d)=r.pop_front().unwrap(); println!("  freed: {}", d); n+=1;
        }
        println!("epoch→{}; freed {}; deferred {}", new_e, n, r.len());
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_epoch_advances() { let m=EpochMgr::new(); m.collect(); assert_eq!(m.epoch.load(Ordering::SeqCst),1); }
    #[test] fn test_retire()        { let m=EpochMgr::new(); m.retire("x"); assert_eq!(m.retired.lock().unwrap().len(),1); }
}
