// 1000: Reactive Stream
// Push-based Observable<T> with map/filter/take/subscribe

use std::cell::RefCell;
use std::rc::Rc;

// --- Observer trait ---
trait Observer<T> {
    fn on_next(&mut self, value: T);
    fn on_error(&mut self, err: &str);
    fn on_complete(&mut self);
}

// --- Simple functional observer ---
struct FnObserver<T> {
    on_next_fn: Box<dyn FnMut(T)>,
    on_complete_fn: Box<dyn FnMut()>,
}

impl<T> FnObserver<T> {
    fn new(on_next: impl FnMut(T) + 'static) -> Self {
        FnObserver {
            on_next_fn: Box::new(on_next),
            on_complete_fn: Box::new(|| {}),
        }
    }

    fn with_complete(mut self, f: impl FnMut() + 'static) -> Self {
        self.on_complete_fn = Box::new(f);
        self
    }
}

impl<T> Observer<T> for FnObserver<T> {
    fn on_next(&mut self, value: T) {
        (self.on_next_fn)(value);
    }
    fn on_error(&mut self, _err: &str) {}
    fn on_complete(&mut self) {
        (self.on_complete_fn)();
    }
}

// --- Observable: a lazy push source ---
struct Observable<T> {
    subscribe_fn: Box<dyn Fn(&mut dyn Observer<T>)>,
}

impl<T: Clone + 'static> Observable<T> {
    fn new(f: impl Fn(&mut dyn Observer<T>) + 'static) -> Self {
        Observable {
            subscribe_fn: Box::new(f),
        }
    }

    fn subscribe(&self, observer: &mut dyn Observer<T>) {
        (self.subscribe_fn)(observer);
    }

    fn from_iter(items: Vec<T>) -> Self {
        Observable::new(move |obs| {
            for item in &items {
                obs.on_next(item.clone());
            }
            obs.on_complete();
        })
    }
}

// --- Operators as free functions (return new Observable) ---

// Adapter structs allow borrowing `observer` without 'static
struct MapAdapter<'a, U, F> {
    inner: &'a mut dyn Observer<U>,
    f: &'a F,
}

impl<'a, T, U, F: Fn(T) -> U> Observer<T> for MapAdapter<'a, U, F> {
    fn on_next(&mut self, value: T) {
        self.inner.on_next((self.f)(value));
    }
    fn on_error(&mut self, err: &str) {
        self.inner.on_error(err);
    }
    fn on_complete(&mut self) {
        self.inner.on_complete();
    }
}

fn obs_map<T: Clone + 'static, U: Clone + 'static>(
    source: Observable<T>,
    f: impl Fn(T) -> U + 'static,
) -> Observable<U> {
    Observable::new(move |observer| {
        let mut adapter = MapAdapter {
            inner: observer,
            f: &f,
        };
        source.subscribe(&mut adapter);
    })
}

struct FilterAdapter<'a, T, P> {
    inner: &'a mut dyn Observer<T>,
    pred: &'a P,
}

impl<'a, T, P: Fn(&T) -> bool> Observer<T> for FilterAdapter<'a, T, P> {
    fn on_next(&mut self, value: T) {
        if (self.pred)(&value) {
            self.inner.on_next(value);
        }
    }
    fn on_error(&mut self, err: &str) {
        self.inner.on_error(err);
    }
    fn on_complete(&mut self) {
        self.inner.on_complete();
    }
}

fn obs_filter<T: Clone + 'static>(
    source: Observable<T>,
    pred: impl Fn(&T) -> bool + 'static,
) -> Observable<T> {
    Observable::new(move |observer| {
        let mut adapter = FilterAdapter {
            inner: observer,
            pred: &pred,
        };
        source.subscribe(&mut adapter);
    })
}

struct TakeAdapter<'a, T> {
    inner: &'a mut dyn Observer<T>,
    remaining: usize,
}

impl<'a, T> Observer<T> for TakeAdapter<'a, T> {
    fn on_next(&mut self, value: T) {
        if self.remaining > 0 {
            self.remaining -= 1;
            self.inner.on_next(value);
        }
    }
    fn on_error(&mut self, err: &str) {
        self.inner.on_error(err);
    }
    fn on_complete(&mut self) {
        self.inner.on_complete();
    }
}

fn obs_take<T: Clone + 'static>(source: Observable<T>, n: usize) -> Observable<T> {
    Observable::new(move |observer| {
        let mut adapter = TakeAdapter {
            inner: observer,
            remaining: n,
        };
        source.subscribe(&mut adapter);
    })
}

// --- Collect all emitted values ---
fn collect<T: Clone + 'static>(source: Observable<T>) -> Vec<T> {
    let results = Rc::new(RefCell::new(Vec::new()));
    let results2 = Rc::clone(&results);
    let mut observer = FnObserver::new(move |v: T| {
        results2.borrow_mut().push(v);
    });
    source.subscribe(&mut observer);
    let x = results.borrow().clone();
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_iter() {
        let obs = Observable::from_iter(vec![1, 2, 3]);
        assert_eq!(collect(obs), vec![1, 2, 3]);
    }

    #[test]
    fn test_map() {
        let obs = Observable::from_iter(vec![1, 2, 3]);
        let mapped = obs_map(obs, |x: i32| x * 2);
        assert_eq!(collect(mapped), vec![2, 4, 6]);
    }

    #[test]
    fn test_filter() {
        let obs = Observable::from_iter(vec![1, 2, 3, 4, 5]);
        let filtered = obs_filter(obs, |x| x % 2 == 0);
        assert_eq!(collect(filtered), vec![2, 4]);
    }

    #[test]
    fn test_take() {
        let obs = Observable::from_iter(vec![1, 2, 3, 4, 5]);
        let taken = obs_take(obs, 3);
        assert_eq!(collect(taken), vec![1, 2, 3]);
    }

    #[test]
    fn test_chain() {
        let source = Observable::from_iter(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let filtered = obs_filter(source, |x| x % 2 == 0);
        let mapped = obs_map(filtered, |x: i32| x * x);
        let taken = obs_take(mapped, 3);
        assert_eq!(collect(taken), vec![4, 16, 36]);
    }

    #[test]
    fn test_empty_observable() {
        let obs: Observable<i32> = Observable::from_iter(vec![]);
        assert_eq!(collect(obs), vec![]);
    }
}
