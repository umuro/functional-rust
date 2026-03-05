fn main() {
    // Basic if let
    let maybe: Option<i32> = Some(42);
    if let Some(v) = maybe { println!("got {}", v); }

    // if let + else
    let r: Result<i32,&str> = Err("oops");
    if let Ok(n) = r { println!("ok: {}", n); } else { println!("failed"); }

    // while let: drain stack
    let mut stack = vec![1,2,3,4,5];
    while let Some(top) = stack.pop() { print!("{} ", top); } println!();

    // while let with iterator
    let mut it = vec!["hello","world"].into_iter();
    while let Some(w) = it.next() { println!("word: {}", w); }

    // if let with enum
    #[derive(Debug)]
    enum Msg { Move{x:i32,y:i32}, Write(String), Quit }
    let msgs = vec![Msg::Move{x:1,y:2}, Msg::Write("hi".into()), Msg::Quit];
    for msg in &msgs {
        if let Msg::Move{x,y} = msg { println!("move to ({},{})", x, y); }
        else if let Msg::Write(t) = msg { println!("write: {}", t); }
    }

    // while let + queue
    let mut q = std::collections::VecDeque::from(["a","b","stop","c"]);
    while let Some(cmd) = q.pop_front() {
        if cmd == "stop" { break; }
        println!("cmd: {}", cmd);
    }
}

#[cfg(test)]
mod tests {
    #[test] fn if_let() { let x: Option<i32>=Some(5); let mut ok=false; if let Some(v)=x { ok=v==5; } assert!(ok); }
    #[test] fn while_let() { let mut v=vec![1,2,3]; let mut s=0; while let Some(x)=v.pop(){s+=x;} assert_eq!(s,6); }
}
