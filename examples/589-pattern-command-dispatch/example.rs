use std::collections::HashMap;

#[derive(Debug,Clone)]
enum Cmd {
    Set(String, i64),
    Remove(String),
    Increment(String, i64),
    Clear,
}

#[derive(Debug,Default)]
struct Store {
    data: HashMap<String,i64>,
    history: Vec<Cmd>,
}

impl Store {
    fn execute(&mut self, cmd: Cmd) {
        match &cmd {
            Cmd::Set(k,v)       => { self.data.insert(k.clone(), *v); }
            Cmd::Remove(k)      => { self.data.remove(k); }
            Cmd::Increment(k,d) => { *self.data.entry(k.clone()).or_default() += d; }
            Cmd::Clear          => { self.data.clear(); }
        }
        self.history.push(cmd);
    }

    fn get(&self, k: &str) -> Option<i64> { self.data.get(k).copied() }
}

// Pure command executor (no side effects)
fn apply(mut data: HashMap<String,i64>, cmd: &Cmd) -> HashMap<String,i64> {
    match cmd {
        Cmd::Set(k,v)       => { data.insert(k.clone(), *v); }
        Cmd::Remove(k)      => { data.remove(k); }
        Cmd::Increment(k,d) => { *data.entry(k.clone()).or_default() += d; }
        Cmd::Clear          => { data.clear(); }
    }
    data
}

fn main() {
    let mut store = Store::default();
    let cmds = vec![
        Cmd::Set("x".into(), 1),
        Cmd::Set("y".into(), 2),
        Cmd::Increment("x".into(), 9),
        Cmd::Remove("y".into()),
        Cmd::Set("z".into(), 3),
    ];
    for cmd in cmds { store.execute(cmd); }
    let mut keys: Vec<_> = store.data.keys().collect();
    keys.sort();
    for k in keys { println!("{}={}", k, store.data[k]); }
    println!("history len={}", store.history.len());

    // Pure replay
    let final_state = store.history.iter().fold(HashMap::new(), |acc, c| apply(acc, c));
    println!("x={:?}", final_state.get("x"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn set_get() {
        let mut s = Store::default();
        s.execute(Cmd::Set("k".into(),42));
        assert_eq!(s.get("k"), Some(42));
    }
    #[test] fn increment() {
        let mut s = Store::default();
        s.execute(Cmd::Increment("n".into(),1));
        s.execute(Cmd::Increment("n".into(),2));
        assert_eq!(s.get("n"), Some(3));
    }
}
