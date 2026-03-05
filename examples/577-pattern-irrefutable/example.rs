// ── Irrefutable patterns ────────────────────────────────────────────────────

fn irrefutable() {
    let x = 42;                          // simple variable
    let (a, b, c) = (1, 2, 3);          // tuple
    struct P { x: f64, y: f64 }
    let P { x, y } = P { x: 1.0, y: 2.0 }; // struct
    fn add((a,b): (i32,i32)) -> i32 { a+b }  // irrefutable param
    let pairs = vec![(1,'a'),(2,'b')];
    for (n, ch) in &pairs { println!("{}{}", n, ch); } // irrefutable for
    println!("{} {} {} {} {} {}", x, a, b, c, add((3,4)), y+x);
}

// ── Refutable patterns ───────────────────────────────────────────────────────
fn refutable() {
    let opt: Option<i32> = Some(42);

    // if let — OK, refutable
    if let Some(v) = opt { println!("Some({})", v); }

    // match — OK, refutable
    match opt { Some(v) => println!("match {}", v), None => {} }

    // while let — OK, refutable
    let mut s = vec![1,2,3];
    while let Some(t) = s.pop() { print!("{} ", t); } println!();

    // This FAILS to compile (uncomment to see error):
    // let Some(v) = opt;   // ERROR: refutable pattern in local binding
}

fn main() { irrefutable(); refutable(); }

#[cfg(test)]
mod tests {
    #[test] fn irr_tuple() { let (a,b) = (10,20); assert_eq!(a+b, 30); }
    #[test] fn irr_struct() {
        struct F { x: i32 }
        let F { x } = F { x: 42 }; assert_eq!(x, 42);
    }
}
