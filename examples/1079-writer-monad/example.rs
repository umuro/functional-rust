#[derive(Debug, Clone)]
struct Writer<A> {
    value: A,
    log: Vec<String>,
}

impl<A> Writer<A> {
    fn new(value: A) -> Self {
        Writer { value, log: Vec::new() }
    }

    fn bind<B, F>(self, f: F) -> Writer<B>
    where F: FnOnce(A) -> Writer<B> {
        let mut result = f(self.value);
        let mut combined = self.log;
        combined.append(&mut result.log);
        Writer { value: result.value, log: combined }
    }

    fn map<B, F>(self, f: F) -> Writer<B>
    where F: FnOnce(A) -> B {
        Writer { value: f(self.value), log: self.log }
    }
}

fn tell(msg: impl Into<String>) -> Writer<()> {
    Writer { value: (), log: vec![msg.into()] }
}

fn half(x: i64) -> Writer<i64> {
    let result = x / 2;
    Writer { value: result, log: vec![format!("halved {x} to {result}")] }
}

fn compute(x: i64) -> Writer<i64> {
    Writer::new(x)
        .bind(half)
        .bind(|n| tell(format!("result is {n}")).map(|()| n))
}

fn main() {
    let result = compute(100);
    println!("Value: {}", result.value);
    for msg in &result.log {
        println!("  Log: {msg}");
    }
}

/* Output:
   Value: 50
     Log: halved 100 to 50
     Log: result is 50
*/
