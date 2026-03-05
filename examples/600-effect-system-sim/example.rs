// Algebraic effects simulation via trait handlers

trait EffectHandler {
    fn log(&mut self, msg: &str);
    fn random(&mut self, lo: i32, hi: i32) -> i32;
    fn get_state(&self) -> i32;
    fn set_state(&mut self, v: i32);
}

// Production handler: real logging, real random, real state
struct ProdHandler { state: i32 }
impl EffectHandler for ProdHandler {
    fn log(&mut self, msg: &str) { println!("[LOG] {}", msg); }
    fn random(&mut self, lo: i32, hi: i32) -> i32 {
        // poor-man's pseudo-random
        (self.state.wrapping_mul(1664525).wrapping_add(1013904223).abs() % (hi-lo) + lo)
    }
    fn get_state(&self) -> i32 { self.state }
    fn set_state(&mut self, v: i32) { self.state = v; }
}

// Test handler: deterministic, captures log
struct TestHandler { state: i32, log: Vec<String>, rand_seq: Vec<i32>, rand_idx: usize }
impl EffectHandler for TestHandler {
    fn log(&mut self, msg: &str) { self.log.push(msg.to_string()); }
    fn random(&mut self, _lo: i32, _hi: i32) -> i32 {
        let v = self.rand_seq[self.rand_idx % self.rand_seq.len()];
        self.rand_idx += 1; v
    }
    fn get_state(&self) -> i32 { self.state }
    fn set_state(&mut self, v: i32) { self.state = v; }
}

// The "program" is generic over the handler — pure business logic
fn simulate(h: &mut dyn EffectHandler) {
    h.log("Starting simulation");
    let v = h.get_state();
    h.set_state(v + 10);
    h.log(&format!("State after +10: {}", h.get_state()));
    let r = h.random(1, 100);
    h.log(&format!("Random roll: {}", r));
    h.set_state(h.get_state() + r);
    h.log(&format!("Final state: {}", h.get_state()));
}

fn main() {
    let mut prod = ProdHandler { state: 0 };
    simulate(&mut prod);

    let mut test = TestHandler { state: 5, log: vec![], rand_seq: vec![42], rand_idx: 0 };
    simulate(&mut test);
    println!("Test logs: {:?}", test.log);
    println!("Test final state: {}", test.state);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_simulate() {
        let mut h = TestHandler { state: 0, log: vec![], rand_seq: vec![7], rand_idx: 0 };
        simulate(&mut h);
        assert_eq!(h.state, 17); // 0+10+7
        assert!(h.log.len() > 0);
    }
}
