#![allow(clippy::all)]
// 986: Mutex-Protected State
// Rust: Mutex<T> owns the data — unlocks automatically via RAII guard

use std::sync::{Arc, Mutex};
use std::thread;

// --- Approach 1: Shared counter with Arc<Mutex<i32>> ---
fn shared_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0i32));

    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..100 {
                let mut n = counter.lock().unwrap();
                *n += 1;
                // Lock released here when `n` drops
            }
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    *counter.lock().unwrap()
}

// --- Approach 2: Mutex around structured state ---
#[derive(Debug)]
struct BankAccount {
    balance: i64,
    transactions: u32,
}

impl BankAccount {
    fn new() -> Self { BankAccount { balance: 0, transactions: 0 } }

    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
        self.transactions += 1;
    }

    fn withdraw(&mut self, amount: i64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            self.transactions += 1;
            true
        } else {
            false
        }
    }
}

fn bank_account_demo() -> (i64, u32) {
    let account = Arc::new(Mutex::new(BankAccount::new()));

    let handles: Vec<_> = (0..5).map(|_| {
        let acct = Arc::clone(&account);
        thread::spawn(move || {
            acct.lock().unwrap().deposit(100);
        })
    }).collect();

    for h in handles { h.join().unwrap(); }

    let mut acct = account.lock().unwrap();
    acct.withdraw(200);
    (acct.balance, acct.transactions)
}

// --- Approach 3: with_mutex helper (bracket / RAII equivalent) ---
fn with_lock<T, R, F: FnOnce(&mut T) -> R>(m: &Mutex<T>, f: F) -> R {
    let mut guard = m.lock().unwrap();
    f(&mut *guard)
    // guard drops here — unlock is automatic
}

fn collect_to_vec() -> Vec<i32> {
    let shared = Arc::new(Mutex::new(Vec::<i32>::new()));

    let handles: Vec<_> = (0..5i32).map(|i| {
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            with_lock(&shared, |v| v.push(i));
        })
    }).collect();

    for h in handles { h.join().unwrap(); }

    let mut v = shared.lock().unwrap().clone();
    v.sort();
    v
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_counter() {
        assert_eq!(shared_counter(), 1000);
    }

    #[test]
    fn test_bank_account() {
        let (balance, txns) = bank_account_demo();
        assert_eq!(balance, 300);  // 5*100 - 200
        assert_eq!(txns, 6);       // 5 deposits + 1 withdrawal
    }

    #[test]
    fn test_collect_to_vec() {
        let v = collect_to_vec();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_mutex_protects_from_data_race() {
        // If counter were unprotected, this would be UB/wrong
        assert_eq!(shared_counter(), 1000);
    }

    #[test]
    fn test_with_lock_helper() {
        let m = Mutex::new(0i32);
        with_lock(&m, |v| *v += 1);
        with_lock(&m, |v| *v += 1);
        assert_eq!(*m.lock().unwrap(), 2);
    }
}
