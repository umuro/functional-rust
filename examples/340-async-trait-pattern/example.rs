use std::future::Future;
use std::pin::Pin;
use std::collections::HashMap;
use std::sync::Mutex;

type AsyncResult<T,E> = Pin<Box<dyn Future<Output=Result<T,E>>+Send>>;

trait AsyncStore: Send+Sync {
    fn get(&self, key: &str) -> AsyncResult<Option<String>, String>;
    fn set(&self, key: String, val: String) -> AsyncResult<(), String>;
}

struct MemStore { data: Mutex<HashMap<String,String>> }
impl MemStore { fn new() -> Self { Self{data:Mutex::new(HashMap::new())} } }

impl AsyncStore for MemStore {
    fn get(&self, key: &str) -> AsyncResult<Option<String>,String> {
        let r = self.data.lock().unwrap().get(key).cloned();
        Box::pin(async move { Ok(r) })
    }
    fn set(&self, key: String, val: String) -> AsyncResult<(),String> {
        self.data.lock().unwrap().insert(key, val);
        Box::pin(async { Ok(()) })
    }
}

struct FailStore;
impl AsyncStore for FailStore {
    fn get(&self, _: &str) -> AsyncResult<Option<String>,String> { Box::pin(async{Err("connection refused".into())}) }
    fn set(&self, _: String, _: String) -> AsyncResult<(),String> { Box::pin(async{Err("read-only".into())}) }
}

fn block_on<F:Future>(fut: F) -> F::Output {
    use std::task::{Context,Poll,RawWaker,RawWakerVTable,Waker};
    unsafe fn c(p:*const())->RawWaker{RawWaker::new(p,&V)} unsafe fn n(_:*const()){}
    static V:RawWakerVTable=RawWakerVTable::new(c,n,n,n);
    let w=unsafe{Waker::from_raw(RawWaker::new(std::ptr::null(),&V))};
    let mut cx=Context::from_waker(&w); let mut f=Box::pin(fut);
    loop{if let Poll::Ready(v)=f.as_mut().poll(&mut cx){return v;}}
}

fn use_store(s: &dyn AsyncStore, k: &str, v: &str) {
    match block_on(s.set(k.into(),v.into())) {
        Err(e) => println!("Set failed: {e}"),
        Ok(()) => match block_on(s.get(k)) {
            Ok(Some(v)) => println!("Got: {v}"),
            Ok(None) => println!("not found"),
            Err(e) => println!("Get failed: {e}"),
        }
    }
}

fn main() {
    use_store(&MemStore::new(), "k", "v");
    use_store(&FailStore, "k", "v");
    let stores: Vec<Box<dyn AsyncStore>> = vec![Box::new(MemStore::new()), Box::new(FailStore)];
    for (i,s) in stores.iter().enumerate() { print!("Store {i}: "); use_store(s.as_ref(),"key","val"); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn mem_get_set() {
        let s = MemStore::new();
        block_on(s.set("k".into(),"v".into())).unwrap();
        assert_eq!(block_on(s.get("k")).unwrap(), Some("v".into()));
    }
    #[test] fn failing_returns_err() {
        let s = FailStore;
        assert!(block_on(s.get("x")).is_err());
    }
}
