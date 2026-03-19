#![allow(clippy::all)]
// 985: Select Pattern — Tokio version
// tokio::select! macro for non-blocking selection over multiple async branches

use tokio::sync::mpsc;

#[derive(Debug, PartialEq)]
enum Selected<A, B> {
    Left(A),
    Right(B),
    BothClosed,
}

/// Select from two channels using tokio::select!
async fn select_drain(
    mut rx1: mpsc::Receiver<i32>,
    mut rx2: mpsc::Receiver<String>,
) -> (Vec<i32>, Vec<String>) {
    let mut lefts = Vec::new();
    let mut rights = Vec::new();

    loop {
        tokio::select! {
            val = rx1.recv() => {
                match val {
                    Some(v) => lefts.push(v),
                    None => {
                        // rx1 closed, drain rx2
                        while let Some(v) = rx2.recv().await {
                            rights.push(v);
                        }
                        break;
                    }
                }
            }
            val = rx2.recv() => {
                match val {
                    Some(v) => rights.push(v),
                    None => {
                        // rx2 closed, drain rx1
                        while let Some(v) = rx1.recv().await {
                            lefts.push(v);
                        }
                        break;
                    }
                }
            }
        }
    }
    (lefts, rights)
}

/// Priority select: biased select prefers first branch
async fn priority_recv(
    high: &mut mpsc::Receiver<i32>,
    low: &mut mpsc::Receiver<i32>,
) -> Option<(i32, bool)> {
    tokio::select! {
        biased;  // always try high first
        val = high.recv() => val.map(|v| (v, true)),
        val = low.recv() => val.map(|v| (v, false)),
    }
}

/// Race: first future to complete wins
async fn race_two() -> &'static str {
    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => "slow",
        _ = tokio::time::sleep(std::time::Duration::from_millis(5)) => "fast",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_select_drain() {
        let (tx1, rx1) = mpsc::channel::<i32>(32);
        let (tx2, rx2) = mpsc::channel::<String>(32);

        for i in [1, 2, 3] { tx1.send(i).await.unwrap(); }
        for s in ["a", "b", "c"] { tx2.send(s.to_string()).await.unwrap(); }
        drop(tx1); drop(tx2);

        let (mut lefts, mut rights) = select_drain(rx1, rx2).await;
        lefts.sort();
        rights.sort();
        assert_eq!(lefts, vec![1, 2, 3]);
        assert_eq!(rights, vec!["a", "b", "c"]);
    }

    #[tokio::test]
    async fn test_priority_recv() {
        let (htx, mut hrx) = mpsc::channel::<i32>(32);
        let (ltx, mut lrx) = mpsc::channel::<i32>(32);

        htx.send(10).await.unwrap();
        ltx.send(20).await.unwrap();

        // High priority wins (biased select)
        let result = priority_recv(&mut hrx, &mut lrx).await;
        assert_eq!(result, Some((10, true)));

        // Now only low available
        let result2 = priority_recv(&mut hrx, &mut lrx).await;
        assert_eq!(result2, Some((20, false)));
    }

    #[tokio::test]
    async fn test_race_two() {
        assert_eq!(race_two().await, "fast");
    }

    #[tokio::test]
    async fn test_both_closed() {
        let (tx1, rx1) = mpsc::channel::<i32>(1);
        let (tx2, rx2) = mpsc::channel::<String>(1);
        drop(tx1); drop(tx2);
        let (lefts, rights) = select_drain(rx1, rx2).await;
        assert!(lefts.is_empty());
        assert!(rights.is_empty());
    }
}
