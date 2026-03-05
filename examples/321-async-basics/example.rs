use std::thread;
use std::time::Duration;

fn fetch_user(id: u32) -> String {
    thread::sleep(Duration::from_millis(10));
    format!("User({id})")
}

fn fetch_posts(user_id: u32) -> Vec<String> {
    thread::sleep(Duration::from_millis(8));
    vec![format!("Post1 by {user_id}"), format!("Post2 by {user_id}")]
}

// Sequential (like: user = fetch_user(id).await; posts = fetch_posts(id).await)
fn sequential_fetch(id: u32) -> (String, Vec<String>) {
    (fetch_user(id), fetch_posts(id))
}

// Concurrent (like: join!(fetch_user(id), fetch_posts(id)))
fn concurrent_fetch(id: u32) -> (String, Vec<String>) {
    let h1 = thread::spawn(move || fetch_user(id));
    let h2 = thread::spawn(move || fetch_posts(id));
    (h1.join().unwrap(), h2.join().unwrap())
}

fn main() {
    let (user, posts) = sequential_fetch(42);
    println!("User: {user}");
    for p in &posts { println!("  {p}"); }
    let (u2, p2) = concurrent_fetch(99);
    println!("Concurrent: {u2} with {} posts", p2.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn sequential_correct() { let (u,_) = sequential_fetch(1); assert_eq!(u, "User(1)"); }
    #[test] fn concurrent_same() { let (u1,p1) = sequential_fetch(7); let (u2,p2) = concurrent_fetch(7); assert_eq!(u1,u2); assert_eq!(p1,p2); }
}
