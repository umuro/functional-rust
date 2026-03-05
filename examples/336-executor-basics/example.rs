use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, mpsc};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

type BoxFuture = Pin<Box<dyn Future<Output=()> + Send>>;

struct Task {
    future: Mutex<Option<BoxFuture>>,
    sender: mpsc::SyncSender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) { let _ = self.sender.send(Arc::clone(self)); }
}

fn make_waker(task: Arc<Task>) -> Waker {
    let ptr = Arc::into_raw(task) as *const ();
    unsafe fn cw(p:*const())->RawWaker{let a=Arc::from_raw(p as *const Task);std::mem::forget(Arc::clone(&a));std::mem::forget(a);RawWaker::new(p,&V)}
    unsafe fn w(p:*const()){Arc::from_raw(p as *const Task).schedule();}
    unsafe fn wbr(p:*const()){let a=Arc::from_raw(p as *const Task);a.schedule();std::mem::forget(a);}
    unsafe fn dw(p:*const()){drop(Arc::from_raw(p as *const Task));}
    static V: RawWakerVTable = RawWakerVTable::new(cw,w,wbr,dw);
    unsafe { Waker::from_raw(RawWaker::new(ptr, &V)) }
}

struct SimpleExecutor { rx: mpsc::Receiver<Arc<Task>>, tx: mpsc::SyncSender<Arc<Task>> }

impl SimpleExecutor {
    fn new() -> Self { let (tx,rx) = mpsc::sync_channel(100); Self{rx,tx} }
    fn spawn(&self, fut: impl Future<Output=()>+Send+'static) {
        let task = Arc::new(Task{future:Mutex::new(Some(Box::pin(fut))),sender:self.tx.clone()});
        task.schedule();
    }
    fn run(self) {
        drop(self.tx);
        while let Ok(task) = self.rx.recv() {
            let mut slot = task.future.lock().unwrap();
            if let Some(mut f) = slot.take() {
                let w = make_waker(Arc::clone(&task));
                let mut cx = Context::from_waker(&w);
                if f.as_mut().poll(&mut cx) == Poll::Pending { *slot = Some(f); }
            }
        }
    }
}

fn main() {
    let ex = SimpleExecutor::new();
    ex.spawn(async { println!("Task A"); });
    ex.spawn(async { println!("Task B"); });
    ex.spawn(async { println!("Task C"); });
    ex.run();
    println!("All done");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize,Ordering};
    #[test] fn runs_tasks() {
        let counter = Arc::new(AtomicUsize::new(0));
        let ex = SimpleExecutor::new();
        for _ in 0..5 { let c = Arc::clone(&counter); ex.spawn(async move { c.fetch_add(1,Ordering::SeqCst); }); }
        ex.run();
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
}
