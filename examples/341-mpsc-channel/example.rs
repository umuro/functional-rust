use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    for i in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("message from worker {i}")).unwrap();
        });
    }

    drop(tx);

    for msg in rx {
        println!("{msg}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collects_all_messages() {
        let (tx, rx) = mpsc::channel::<i32>();
        for i in 0..5 {
            let tx = tx.clone();
            thread::spawn(move || { tx.send(i).unwrap(); });
        }
        drop(tx);
        let msgs: Vec<_> = rx.into_iter().collect();
        assert_eq!(msgs.len(), 5);
    }
}
