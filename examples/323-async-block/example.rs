fn lazy_comp<F: FnOnce() -> T, T>(label: &str, f: F) -> impl FnOnce() -> T + '_ {
    println!("Creating: {label}");
    move || { println!("Executing: {label}"); f() }
}

fn run_if<F: FnOnce() -> T, T>(cond: bool, t: F) -> Option<T> {
    if cond { Some(t()) } else { None }
}

fn main() {
    let t1 = lazy_comp("double(5)", || 5*2);
    let t2 = lazy_comp("square(4)", || 4*4);
    println!("Result1: {}", t1());
    println!("Result2: {}", t2());
    let r = run_if(false, lazy_comp("expensive", || 9999_i32.pow(3)));
    println!("Cond: {}", r.map_or("skipped".into(), |v: i32| v.to_string()));
    // Capture by move (like async move {})
    let mult = 7i32;
    let tasks: Vec<Box<dyn FnOnce() -> i32>> = (1..=5)
        .map(|x| -> Box<dyn FnOnce()->i32> { Box::new(move || x * mult) })
        .collect();
    for t in tasks { print!("{} ", t()); }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn not_called_until_invoked() {
        let mut called = false;
        let t = || { called = true; 42 };
        assert!(!called); let v = t(); assert!(called); assert_eq!(v, 42);
    }
    #[test] fn run_if_skips() { assert!(run_if(false, || panic!("no")).is_none()); }
    #[test] fn run_if_runs() { assert_eq!(run_if(true, || 42), Some(42)); }
}
