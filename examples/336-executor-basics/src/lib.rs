#![allow(clippy::all)]
//! # Executor Basics
//!
//! A minimal async executor — the engine that drives futures to completion by polling them.

use std::future::Future;
use std::pin::Pin;
use std::sync::{mpsc, Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

struct Task {
    future: Mutex<Option<BoxFuture>>,
    sender: mpsc::SyncSender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        let _ = self.sender.send(Arc::clone(self));
    }
}

fn make_waker(task: Arc<Task>) -> Waker {
    let ptr = Arc::into_raw(task) as *const ();

    unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
        let arc = Arc::from_raw(ptr as *const Task);
        std::mem::forget(Arc::clone(&arc));
        std::mem::forget(arc);
        RawWaker::new(ptr, &VTABLE)
    }

    unsafe fn wake(ptr: *const ()) {
        Arc::from_raw(ptr as *const Task).schedule();
    }

    unsafe fn wake_by_ref(ptr: *const ()) {
        let arc = Arc::from_raw(ptr as *const Task);
        arc.schedule();
        std::mem::forget(arc);
    }

    unsafe fn drop_waker(ptr: *const ()) {
        drop(Arc::from_raw(ptr as *const Task));
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_waker, wake, wake_by_ref, drop_waker);

    unsafe { Waker::from_raw(RawWaker::new(ptr, &VTABLE)) }
}

/// A simple single-threaded executor.
pub struct SimpleExecutor {
    rx: mpsc::Receiver<Arc<Task>>,
    tx: mpsc::SyncSender<Arc<Task>>,
}

impl SimpleExecutor {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::sync_channel(100);
        Self { rx, tx }
    }

    pub fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(fut))),
            sender: self.tx.clone(),
        });
        task.schedule();
    }

    pub fn run(self) {
        drop(self.tx); // Drop sender so rx.recv() ends when all tasks complete

        while let Ok(task) = self.rx.recv() {
            let mut slot = task.future.lock().unwrap();
            if let Some(mut fut) = slot.take() {
                let waker = make_waker(Arc::clone(&task));
                let mut cx = Context::from_waker(&waker);
                if fut.as_mut().poll(&mut cx) == Poll::Pending {
                    *slot = Some(fut);
                }
            }
        }
    }
}

impl Default for SimpleExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_runs_single_task() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = Arc::clone(&counter);

        let ex = SimpleExecutor::new();
        ex.spawn(async move {
            c.fetch_add(1, Ordering::SeqCst);
        });
        ex.run();

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_runs_multiple_tasks() {
        let counter = Arc::new(AtomicUsize::new(0));

        let ex = SimpleExecutor::new();
        for _ in 0..5 {
            let c = Arc::clone(&counter);
            ex.spawn(async move {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }
        ex.run();

        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }

    #[test]
    fn test_empty_executor() {
        let ex = SimpleExecutor::new();
        ex.run(); // Should complete immediately
    }

    #[test]
    fn test_tasks_run_to_completion() {
        let values = Arc::new(Mutex::new(Vec::new()));

        let ex = SimpleExecutor::new();
        for i in 0..3 {
            let v = Arc::clone(&values);
            ex.spawn(async move {
                v.lock().unwrap().push(i);
            });
        }
        ex.run();

        let mut result = values.lock().unwrap().clone();
        result.sort();
        assert_eq!(result, vec![0, 1, 2]);
    }
}
