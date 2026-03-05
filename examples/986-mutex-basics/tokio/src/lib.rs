// 986: Mutex-Protected State — Tokio version
// tokio::sync::Mutex — async-aware mutex that can be held across .await

use std::sync::Arc;
use tokio::sync::Mutex;

/// Shared counter with Arc<tokio::sync::Mutex<i32>>
async fn shared_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0i32));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);
            tokio::spawn(async move {
                for _ in 0..100 {
                    let mut n = counter.lock().await;
                    *n += 1;
                    // Lock released when `n` drops
                }
            })
        })
        .collect();

    for h in handles {
        h.await.unwrap();
    }
    *counter.lock().await
}

/// Mutex around structured state
#[derive(Debug)]
struct BankAccount {
    balance: i64,
    transactions: u32,
}

impl BankAccount {
    fn new() -> Self { BankAccount { balance: 0, transactions: 0 } }
    fn deposit(&mut self, amount: i64) { self.balance += amount; self.transactions += 1; }
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

async fn bank_account_demo() -> (i64, u32) {
    let account = Arc::new(Mutex::new(BankAccount::new()));

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let acct = Arc::clone(&account);
            tokio::spawn(async move {
                acct.lock().await.deposit(100);
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }

    let mut acct = account.lock().await;
    acct.withdraw(200);
    (acct.balance, acct.transactions)
}

/// with_lock helper using async closure pattern
async fn collect_to_vec() -> Vec<i32> {
    let shared = Arc::new(Mutex::new(Vec::<i32>::new()));

    let handles: Vec<_> = (0..5i32)
        .map(|i| {
            let shared = Arc::clone(&shared);
            tokio::spawn(async move {
                shared.lock().await.push(i);
            })
        })
        .collect();

    for h in handles { h.await.unwrap(); }

    let mut v = shared.lock().await.clone();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shared_counter() {
        assert_eq!(shared_counter().await, 1000);
    }

    #[tokio::test]
    async fn test_bank_account() {
        let (balance, txns) = bank_account_demo().await;
        assert_eq!(balance, 300);  // 5*100 - 200
        assert_eq!(txns, 6);       // 5 deposits + 1 withdrawal
    }

    #[tokio::test]
    async fn test_collect_to_vec() {
        let v = collect_to_vec().await;
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_mutex_protects_from_data_race() {
        assert_eq!(shared_counter().await, 1000);
    }
}
