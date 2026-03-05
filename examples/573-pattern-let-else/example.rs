fn parse_port(s: &str) -> Option<u16> {
    s.parse().ok().filter(|&p: &u16| p > 0)
}

fn parse_config(line: &str) -> Option<(String, u16)> {
    let parts: Vec<&str> = line.split(':').collect();
    let [host, port_str] = parts.as_slice() else { return None; };
    let Ok(port) = port_str.parse::<u16>() else { return None; };
    Some((host.to_string(), port))
}

// let-else in a loop
fn sum_valid(inputs: &[&str]) -> i32 {
    let mut total = 0;
    for &s in inputs {
        let Ok(n) = s.parse::<i32>() else {
            eprintln!("skip: {}", s); continue;
        };
        total += n;
    }
    total
}

// let-else with complex enum
#[derive(Debug)]
struct User { id: u64, name: String, admin: bool }

fn admin_name(users: &[User], id: u64) -> Option<&str> {
    let Some(u) = users.iter().find(|u| u.id == id) else { return None; };
    if !u.admin { return None; }
    Some(&u.name)
}

fn main() {
    for line in ["localhost:8080","bad","host:notaport","example.com:443"] {
        match parse_config(line) {
            Some((h,p)) => println!("-> {}:{}", h, p),
            None        => println!("invalid: {}", line),
        }
    }
    println!("sum valid: {}", sum_valid(&["1","two","3","four","5"]));
    let users = vec![
        User{id:1,name:"Alice".into(),admin:true},
        User{id:2,name:"Bob".into(),  admin:false},
    ];
    println!("admin 1: {:?}", admin_name(&users, 1));
    println!("admin 2: {:?}", admin_name(&users, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_parse() { assert!(parse_config("h:80").is_some()); assert!(parse_config("bad").is_none()); }
    #[test] fn test_sum()   { assert_eq!(sum_valid(&["1","x","2"]), 3); }
}
