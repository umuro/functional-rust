// Profunctor in Rust

/// Dimap: pre and post compose
fn dimap<A, B, C, D>(
    pre: impl FnOnce(C) -> A,
    post: impl FnOnce(B) -> D,
    f: impl FnOnce(A) -> B,
) -> impl FnOnce(C) -> D {
    move |c| post(f(pre(c)))
}

fn main() {
    // Original function: i32 -> String
    let f = |x: i32| x.to_string();
    
    // dimap: &str -> i32 (len) -> String -> usize (len)
    let g = dimap(
        |s: &str| s.len() as i32,
        |s: String| s.len(),
        f
    );
    
    let result = g("hello");
    println!("Result: {}", result); // 1 (len of "5")
    
    // Practical: adapting API boundaries
    let parse = |s: &str| s.parse::<i32>().unwrap_or(0);
    let format_result = dimap(
        |input: String| input.as_str().to_owned(),
        |n: i32| format!("Parsed: {}", n),
        |s: String| s.parse::<i32>().unwrap_or(0)
    );
}
