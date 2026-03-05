fn classify_cmd(s: &str) -> &'static str {
    match s {
        "quit"|"exit"|"q"  => "quit",
        "help"|"?"|"h"     => "help",
        s if s.starts_with('/') => "command",
        ""                 => "empty",
        _                  => "unknown",
    }
}

fn day_type(d: &str) -> &'static str {
    match d {
        "Monday"|"Tuesday"|"Wednesday"|"Thursday"|"Friday" => "weekday",
        "Saturday"|"Sunday"                                => "weekend",
        _                                                  => "unknown",
    }
}

fn http_method(m: &str) -> &'static str {
    match m {
        "GET"               => "read",
        "POST"|"PUT"|"PATCH"=> "write",
        "DELETE"            => "delete",
        _                   => "unknown",
    }
}

fn greet(name: &str) -> String {
    match name {
        "Alice"                   => "Hello, Admin Alice!".into(),
        ""                        => "Hello, stranger!".into(),
        n if n.starts_with("Dr.") => format!("Good day, {}!", n),
        n                         => format!("Hi, {}!", n),
    }
}

fn main() {
    for s in ["quit","help","/run","","foo"] { println!("'{}'->{}", s, classify_cmd(s)); }
    for d in ["Monday","Saturday","Holiday"] { println!("{}:{}", d, day_type(d)); }
    for m in ["GET","POST","DELETE","WUT"]   { println!("{}:{}", m, http_method(m)); }
    for n in ["Alice","Dr.Smith","","Bob"]   { println!("{}", greet(n)); }

    // Matching on &String via deref
    let owned = String::from("Monday");
    println!("owned: {}", day_type(&owned));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn cmd_quit() { assert_eq!(classify_cmd("quit"), "quit"); }
    #[test] fn cmd_slash() { assert_eq!(classify_cmd("/run"), "command"); }
    #[test] fn day_weekday() { assert_eq!(day_type("Monday"), "weekday"); }
}
