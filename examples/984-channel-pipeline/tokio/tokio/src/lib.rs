// 984: Channel Pipeline — Tokio version
// Chain of async processing stages via tokio::sync::mpsc

use tokio::sync::mpsc;

/// Build a pipeline stage: read from rx, apply f, send to tx
fn pipeline_stage<T, U, F>(
    mut rx: mpsc::Receiver<T>,
    f: F,
) -> mpsc::Receiver<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + 'static,
{
    let (tx_out, rx_out) = mpsc::channel(32);
    tokio::spawn(async move {
        while let Some(item) = rx.recv().await {
            if tx_out.send(f(item)).await.is_err() {
                break;
            }
        }
    });
    rx_out
}

/// Build a full 3-stage pipeline
async fn run_pipeline(inputs: Vec<i32>) -> Vec<String> {
    let (tx_source, rx0) = mpsc::channel::<i32>(32);

    // Stage 1: double
    let rx1 = pipeline_stage(rx0, |x| x * 2);
    // Stage 2: add 1
    let rx2 = pipeline_stage(rx1, |x| x + 1);
    // Stage 3: to string
    let mut rx3 = pipeline_stage(rx2, |x: i32| x.to_string());

    // Producer
    tokio::spawn(async move {
        for v in inputs {
            tx_source.send(v).await.unwrap();
        }
    });

    let mut results = Vec::new();
    while let Some(v) = rx3.recv().await {
        results.push(v);
    }
    results
}

/// Parameterised N-stage pipeline
async fn run_n_stages(
    inputs: Vec<i32>,
    stages: Vec<Box<dyn Fn(i32) -> i32 + Send + 'static>>,
) -> Vec<i32> {
    let (tx_source, mut current_rx) = mpsc::channel::<i32>(32);

    for f in stages {
        current_rx = pipeline_stage(current_rx, f);
    }

    tokio::spawn(async move {
        for v in inputs {
            tx_source.send(v).await.unwrap();
        }
    });

    let mut results = Vec::new();
    while let Some(v) = current_rx.recv().await {
        results.push(v);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pipeline_3_stages() {
        let results = run_pipeline(vec![1, 2, 3, 4, 5]).await;
        assert_eq!(results, vec!["3", "5", "7", "9", "11"]);
    }

    #[tokio::test]
    async fn test_pipeline_empty() {
        let results = run_pipeline(vec![]).await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_pipeline_single_item() {
        let results = run_pipeline(vec![5]).await;
        assert_eq!(results, vec!["11"]); // 5*2=10, 10+1=11
    }

    #[tokio::test]
    async fn test_n_stage_pipeline() {
        let stages: Vec<Box<dyn Fn(i32) -> i32 + Send + 'static>> = vec![
            Box::new(|x| x + 10),
            Box::new(|x| x * 3),
            Box::new(|x| x - 1),
        ];
        let results = run_n_stages(vec![1], stages).await;
        assert_eq!(results, vec![32]); // 1+10=11, 11*3=33, 33-1=32
    }

    #[tokio::test]
    async fn test_stage_closure() {
        let (tx, rx) = mpsc::channel::<i32>(32);
        let mut rx_out = pipeline_stage(rx, |x| x * x);

        tokio::spawn(async move {
            for v in [2, 3, 4] {
                tx.send(v).await.unwrap();
            }
        });

        let mut results = Vec::new();
        while let Some(v) = rx_out.recv().await {
            results.push(v);
        }
        assert_eq!(results, vec![4, 9, 16]);
    }
}
