// 984: Channel Pipeline
// Chain of processing stages via mpsc channels

use std::sync::mpsc;
use std::thread;

// --- Build a pipeline stage: read from rx, apply f, send to tx ---
fn pipeline_stage<T, U, F>(rx: mpsc::Receiver<T>, f: F) -> mpsc::Receiver<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + 'static,
{
    let (tx_out, rx_out) = mpsc::channel();
    thread::spawn(move || {
        for item in rx.iter() {      // iter() stops when channel closes
            tx_out.send(f(item)).unwrap();
        }
        // tx_out drops here → closes next stage
    });
    rx_out
}

// --- Build a full pipeline from a Vec of boxed functions ---
fn run_pipeline(inputs: Vec<i32>) -> Vec<String> {
    let (tx_source, rx0) = mpsc::channel::<i32>();

    // Stage 1: double
    let rx1 = pipeline_stage(rx0, |x| x * 2);
    // Stage 2: add 1
    let rx2 = pipeline_stage(rx1, |x| x + 1);
    // Stage 3: to string
    let rx3 = pipeline_stage(rx2, |x: i32| x.to_string());

    // Producer
    let producer = thread::spawn(move || {
        for v in inputs {
            tx_source.send(v).unwrap();
        }
        // tx_source drops → closes pipeline
    });

    // Collect results
    let results: Vec<String> = rx3.iter().collect();
    producer.join().unwrap();
    results
}

// --- Parameterised N-stage pipeline ---
fn run_n_stages(inputs: Vec<i32>, stages: Vec<Box<dyn Fn(i32) -> i32 + Send + 'static>>) -> Vec<i32> {
    let (tx_source, mut current_rx) = mpsc::channel::<i32>();

    for f in stages {
        current_rx = pipeline_stage(current_rx, f);
    }

    let producer = thread::spawn(move || {
        for v in inputs {
            tx_source.send(v).unwrap();
        }
    });

    let results: Vec<i32> = current_rx.iter().collect();
    producer.join().unwrap();
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_3_stages() {
        let results = run_pipeline(vec![1, 2, 3, 4, 5]);
        // 1->2->3, 2->4->5, 3->6->7, 4->8->9, 5->10->11
        assert_eq!(results, vec!["3", "5", "7", "9", "11"]);
    }

    #[test]
    fn test_pipeline_empty() {
        let results = run_pipeline(vec![]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_pipeline_single_item() {
        let results = run_pipeline(vec![5]);
        assert_eq!(results, vec!["11"]); // 5*2=10, 10+1=11
    }

    #[test]
    fn test_n_stage_pipeline() {
        // +10, *3, -1: 1->11->33->32
        let stages: Vec<Box<dyn Fn(i32) -> i32 + Send + 'static>> = vec![
            Box::new(|x| x + 10),
            Box::new(|x| x * 3),
            Box::new(|x| x - 1),
        ];
        let results = run_n_stages(vec![1], stages);
        assert_eq!(results, vec![32]);
    }

    #[test]
    fn test_stage_closure() {
        let (tx, rx) = mpsc::channel::<i32>();
        let rx_out = pipeline_stage(rx, |x| x * x);

        let h = thread::spawn(move || {
            for v in [2, 3, 4] { tx.send(v).unwrap(); }
        });
        h.join().unwrap();

        let results: Vec<i32> = rx_out.iter().collect();
        assert_eq!(results, vec![4, 9, 16]);
    }
}
