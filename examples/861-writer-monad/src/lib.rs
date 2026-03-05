// Example 062: Writer Monad
// Accumulate a log alongside computation results

// Approach 1: Writer struct with Vec<String> log
#[derive(Debug, Clone)]
struct Writer<A> {
    value: A,
    log: Vec<String>,
}

impl<A> Writer<A> {
    fn pure(a: A) -> Self {
        Writer { value: a, log: vec![] }
    }

    fn tell(msg: String) -> Writer<()> {
        Writer { value: (), log: vec![msg] }
    }

    fn and_then<B>(self, f: impl FnOnce(A) -> Writer<B>) -> Writer<B> {
        let Writer { value: b, log: log2 } = f(self.value);
        let mut log = self.log;
        log.extend(log2);
        Writer { value: b, log }
    }

    fn map<B>(self, f: impl FnOnce(A) -> B) -> Writer<B> {
        Writer { value: f(self.value), log: self.log }
    }
}

fn add_with_log(x: i32, y: i32) -> Writer<i32> {
    Writer::tell(format!("Adding {} + {}", x, y))
        .and_then(move |()| {
            let sum = x + y;
            Writer::tell(format!("Result: {}", sum))
                .map(move |()| sum)
        })
}

fn multiply_with_log(x: i32, y: i32) -> Writer<i32> {
    Writer::tell(format!("Multiplying {} * {}", x, y))
        .map(move |()| x * y)
}

fn computation() -> Writer<i32> {
    add_with_log(3, 4)
        .and_then(|sum| multiply_with_log(sum, 2))
        .and_then(|product| {
            Writer::tell("Done!".to_string()).map(move |()| product)
        })
}

// Approach 2: Generic Writer with any monoid-like log
#[derive(Debug)]
struct WriterG<W, A> {
    value: A,
    log: W,
}

impl<A> WriterG<String, A> {
    fn str_pure(a: A) -> Self {
        WriterG { value: a, log: String::new() }
    }

    fn str_tell(msg: &str) -> WriterG<String, ()> {
        WriterG { value: (), log: msg.to_string() }
    }

    fn str_bind<B>(self, f: impl FnOnce(A) -> WriterG<String, B>) -> WriterG<String, B> {
        let w2 = f(self.value);
        WriterG { value: w2.value, log: self.log + &w2.log }
    }
}

// Approach 3: Collect values (Writer as accumulator)
fn gather_evens(xs: &[i32]) -> Writer<()> {
    xs.iter().fold(Writer::pure(()), |acc, &x| {
        acc.and_then(move |()| {
            if x % 2 == 0 {
                Writer { value: (), log: vec![format!("{}", x)] }
            } else {
                Writer::pure(())
            }
        })
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computation() {
        let w = computation();
        assert_eq!(w.value, 14);
        assert_eq!(w.log.len(), 3);
        assert!(w.log[0].contains("Adding 3 + 4"));
    }

    #[test]
    fn test_pure_empty_log() {
        let w = Writer::pure(42);
        assert_eq!(w.value, 42);
        assert!(w.log.is_empty());
    }

    #[test]
    fn test_tell() {
        let w = Writer::tell("hello".into());
        assert_eq!(w.log, vec!["hello"]);
    }

    #[test]
    fn test_gather_evens() {
        let w = gather_evens(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(w.log, vec!["2", "4", "6"]);
    }

    #[test]
    fn test_map() {
        let w = Writer::pure(5).map(|x| x * 2);
        assert_eq!(w.value, 10);
        assert!(w.log.is_empty());
    }

    #[test]
    fn test_and_then_combines_logs() {
        let w = Writer::tell("a".into())
            .and_then(|()| Writer::tell("b".into()));
        assert_eq!(w.log, vec!["a", "b"]);
    }
}
