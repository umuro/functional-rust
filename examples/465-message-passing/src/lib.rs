#![allow(clippy::all)]
// 465. Message passing vs shared memory
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn count_words(s: &str) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in s.split_whitespace() {
        *m.entry(w.to_lowercase()).or_insert(0) += 1;
    }
    m
}

fn merge(mut a: HashMap<String, usize>, b: HashMap<String, usize>) -> HashMap<String, usize> {
    for (k, v) in b {
        *a.entry(k).or_insert(0) += v;
    }
    a
}

fn msg_passing(texts: Vec<String>) -> HashMap<String, usize> {
    let (tx, rx) = mpsc::channel::<HashMap<String, usize>>();
    for t in texts {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(count_words(&t)).unwrap();
        });
    }
    drop(tx);
    rx.iter().fold(HashMap::new(), merge)
}

fn shared_mem(texts: Vec<String>) -> HashMap<String, usize> {
    let shared = Arc::new(Mutex::new(HashMap::<String, usize>::new()));
    let hs: Vec<_> = texts
        .into_iter()
        .map(|t| {
            let s = Arc::clone(&shared);
            thread::spawn(move || {
                let local = count_words(&t);
                let mut g = s.lock().unwrap();
                for (k, v) in local {
                    *g.entry(k).or_insert(0) += v;
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    Arc::try_unwrap(shared).unwrap().into_inner().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_same() {
        let t = vec!["a b c".to_string(), "a d".to_string()];
        assert_eq!(msg_passing(t.clone()), shared_mem(t));
    }
}
