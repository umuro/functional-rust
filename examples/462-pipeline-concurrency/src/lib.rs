//! # Pipeline Concurrency — Staged Processing
//!
//! Process data through multiple stages, each running in its own thread.

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

/// A pipeline stage
pub struct Stage<I, O> {
    handle: JoinHandle<()>,
    _phantom: std::marker::PhantomData<(I, O)>,
}

/// Build a processing pipeline
pub struct Pipeline<T> {
    sender: Sender<T>,
    handles: Vec<JoinHandle<()>>,
}

impl<T: Send + 'static> Pipeline<T> {
    /// Create the first stage of the pipeline
    pub fn new<F, O>(processor: F) -> (Self, Receiver<O>)
    where
        F: Fn(T) -> O + Send + 'static,
        O: Send + 'static,
    {
        let (input_tx, input_rx) = mpsc::channel::<T>();
        let (output_tx, output_rx) = mpsc::channel::<O>();

        let handle = thread::spawn(move || {
            for item in input_rx {
                let result = processor(item);
                if output_tx.send(result).is_err() {
                    break;
                }
            }
        });

        (
            Pipeline {
                sender: input_tx,
                handles: vec![handle],
            },
            output_rx,
        )
    }

    /// Send an item into the pipeline
    pub fn send(&self, item: T) -> Result<(), mpsc::SendError<T>> {
        self.sender.send(item)
    }

    /// Close input and wait for completion
    pub fn finish(self) {
        drop(self.sender);
        for h in self.handles {
            let _ = h.join();
        }
    }
}

/// Add a stage to a receiver
pub fn add_stage<I, O, F>(input: Receiver<I>, processor: F) -> Receiver<O>
where
    I: Send + 'static,
    O: Send + 'static,
    F: Fn(I) -> O + Send + 'static,
{
    let (output_tx, output_rx) = mpsc::channel();

    thread::spawn(move || {
        for item in input {
            let result = processor(item);
            if output_tx.send(result).is_err() {
                break;
            }
        }
    });

    output_rx
}

/// Simple three-stage pipeline example
pub fn three_stage_pipeline(input: Vec<i32>) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();

    // Stage 1: double
    let rx = add_stage(rx, |x| x * 2);

    // Stage 2: add 1
    let rx = add_stage(rx, |x| x + 1);

    // Stage 3: square
    let rx = add_stage(rx, |x| x * x);

    // Send input
    for item in input {
        tx.send(item).unwrap();
    }
    drop(tx);

    // Collect output
    rx.iter().collect()
}

/// Filter-map pipeline
pub fn filter_map_pipeline<T, U, F, P>(
    input: Vec<T>,
    predicate: P,
    mapper: F,
) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    P: Fn(&T) -> bool + Send + 'static,
    F: Fn(T) -> U + Send + 'static,
{
    let (tx, rx) = mpsc::channel();

    // Filter stage
    let (filter_tx, filter_rx) = mpsc::channel();
    thread::spawn(move || {
        for item in rx {
            if predicate(&item) {
                if filter_tx.send(item).is_err() {
                    break;
                }
            }
        }
    });

    // Map stage
    let output_rx = add_stage(filter_rx, mapper);

    // Send input
    for item in input {
        tx.send(item).unwrap();
    }
    drop(tx);

    output_rx.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pipeline() {
        let (pipeline, output) = Pipeline::new(|x: i32| x * 2);

        pipeline.send(1).unwrap();
        pipeline.send(2).unwrap();
        pipeline.send(3).unwrap();
        pipeline.finish();

        let results: Vec<_> = output.iter().collect();
        assert_eq!(results, vec![2, 4, 6]);
    }

    #[test]
    fn test_three_stage() {
        // For input [1, 2]: double -> +1 -> square
        // 1 -> 2 -> 3 -> 9
        // 2 -> 4 -> 5 -> 25
        let results = three_stage_pipeline(vec![1, 2]);
        assert_eq!(results, vec![9, 25]);
    }

    #[test]
    fn test_add_stage() {
        let (tx, rx) = mpsc::channel();

        let rx = add_stage(rx, |x: i32| x.to_string());

        tx.send(42).unwrap();
        tx.send(100).unwrap();
        drop(tx);

        let results: Vec<_> = rx.iter().collect();
        assert_eq!(results, vec!["42", "100"]);
    }

    #[test]
    fn test_filter_map_pipeline() {
        let input = vec![1, 2, 3, 4, 5, 6];
        // Keep evens, then square
        let results = filter_map_pipeline(input, |x| x % 2 == 0, |x| x * x);
        assert_eq!(results, vec![4, 16, 36]);
    }

    #[test]
    fn test_empty_input() {
        let results = three_stage_pipeline(vec![]);
        assert!(results.is_empty());
    }
}
