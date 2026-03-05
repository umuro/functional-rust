use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug,Clone)]
enum RetryError<E> { Transient(E), Permanent(E) }

struct RetryConfig { max_attempts: usize, base_delay: Duration, multiplier: f64 }

impl Default for RetryConfig {
    fn default() -> Self { Self { max_attempts: 3, base_delay: Duration::from_millis(5), multiplier: 2.0 } }
}

fn retry<T, E: Clone>(cfg: &RetryConfig, mut f: impl FnMut()->Result<T,RetryError<E>>) -> Result<T, E> {
    let mut delay = cfg.base_delay;
    let mut last = None;
    for attempt in 1..=cfg.max_attempts {
        match f() {
            Ok(v) => return Ok(v),
            Err(RetryError::Permanent(e)) => return Err(e),
            Err(RetryError::Transient(e)) => {
                last = Some(e);
                if attempt < cfg.max_attempts {
                    println!("Attempt {attempt} failed, retrying in {}ms", delay.as_millis());
                    thread::sleep(delay);
                    delay = delay.mul_f64(cfg.multiplier);
                }
            }
        }
    }
    Err(last.unwrap())
}

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let c = Arc::clone(&counter);
    let result = retry(&RetryConfig::default(), move || {
        let n = c.fetch_add(1, Ordering::SeqCst);
        if n < 2 { Err(RetryError::Transient(format!("not ready ({})", n+1))) } else { Ok(42) }
    });
    println!("Success after {} attempts: {:?}", counter.load(Ordering::SeqCst), result);

    let result: Result<i32, String> = retry(&RetryConfig{max_attempts:5,..Default::default()}, || Err(RetryError::Permanent("fatal".into())));
    println!("Permanent: {result:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn succeeds_after_retries() {
        let c = Arc::new(AtomicUsize::new(0));
        let cc = Arc::clone(&c);
        let cfg = RetryConfig{base_delay:Duration::from_millis(1),..Default::default()};
        let r: Result<i32,String> = retry(&cfg, move || {
            let n = cc.fetch_add(1,Ordering::SeqCst);
            if n<2 { Err(RetryError::Transient("nope".into())) } else { Ok(99) }
        });
        assert_eq!(r.unwrap(), 99);
        assert_eq!(c.load(Ordering::SeqCst), 3);
    }
    #[test] fn permanent_no_retry() {
        let c = Arc::new(AtomicUsize::new(0));
        let cc = Arc::clone(&c);
        let _: Result<i32,String> = retry(&RetryConfig::default(), move || { cc.fetch_add(1,Ordering::SeqCst); Err(RetryError::Permanent("fatal".into())) });
        assert_eq!(c.load(Ordering::SeqCst), 1);
    }
}
