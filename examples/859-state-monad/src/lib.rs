#![allow(clippy::all)]
// Example 060: State Monad
// Thread state through computations without explicit passing

// State monad: S -> (A, S)
struct State<S, A> {
    run: Box<dyn FnOnce(S) -> (A, S)>,
}

impl<S: 'static, A: 'static> State<S, A> {
    fn new(f: impl FnOnce(S) -> (A, S) + 'static) -> Self {
        State { run: Box::new(f) }
    }

    fn run(self, s: S) -> (A, S) {
        (self.run)(s)
    }

    fn pure(a: A) -> Self {
        State::new(move |s| (a, s))
    }

    fn and_then<B: 'static>(self, f: impl FnOnce(A) -> State<S, B> + 'static) -> State<S, B> {
        State::new(move |s| {
            let (a, s2) = self.run(s);
            f(a).run(s2)
        })
    }

    fn map<B: 'static>(self, f: impl FnOnce(A) -> B + 'static) -> State<S, B> {
        State::new(move |s| {
            let (a, s2) = self.run(s);
            (f(a), s2)
        })
    }
}

fn get<S: Clone + 'static>() -> State<S, S> {
    State::new(|s: S| (s.clone(), s))
}

fn put<S: 'static>(new_s: S) -> State<S, ()> {
    State::new(move |_| ((), new_s))
}

fn modify<S: 'static>(f: impl FnOnce(S) -> S + 'static) -> State<S, ()> {
    State::new(move |s| ((), f(s)))
}

// Approach 1: Counter
fn tick() -> State<i32, i32> {
    get::<i32>().and_then(|n| put(n + 1).map(move |()| n))
}

fn count3() -> State<i32, (i32, i32, i32)> {
    tick().and_then(|a| tick().and_then(move |b| tick().map(move |c| (a, b, c))))
}

// Approach 2: Explicit state threading (no State monad — idiomatic Rust)
fn count3_explicit(state: i32) -> ((i32, i32, i32), i32) {
    let a = state;
    let state = state + 1;
    let b = state;
    let state = state + 1;
    let c = state;
    let state = state + 1;
    ((a, b, c), state)
}

// Approach 3: Stack operations
fn push(x: i32) -> State<Vec<i32>, ()> {
    modify(move |mut stack: Vec<i32>| {
        stack.push(x);
        stack
    })
}

fn pop() -> State<Vec<i32>, Option<i32>> {
    State::new(|mut stack: Vec<i32>| {
        let val = stack.pop();
        (val, stack)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let (result, state) = count3().run(0);
        assert_eq!(result, (0, 1, 2));
        assert_eq!(state, 3);
    }

    #[test]
    fn test_counter_nonzero_start() {
        let (result, state) = count3().run(10);
        assert_eq!(result, (10, 11, 12));
        assert_eq!(state, 13);
    }

    #[test]
    fn test_explicit_same_as_monadic() {
        let (r1, s1) = count3().run(0);
        let (r2, s2) = count3_explicit(0);
        assert_eq!(r1, r2);
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_stack_push_pop() {
        let ops = push(10).and_then(|()| push(20)).and_then(|()| pop());
        let (val, stack) = ops.run(vec![]);
        assert_eq!(val, Some(20));
        assert_eq!(stack, vec![10]);
    }

    #[test]
    fn test_stack_pop_empty() {
        let (val, stack) = pop().run(vec![]);
        assert_eq!(val, None);
        assert_eq!(stack, Vec::<i32>::new());
    }

    #[test]
    fn test_pure() {
        let (val, state) = State::<i32, _>::pure(42).run(0);
        assert_eq!(val, 42);
        assert_eq!(state, 0);
    }
}
