struct Config { host: String, port: u16, timeout: f64, debug: bool }

fn connect(Config { host, port, .. }: &Config) {
    println!("Connecting to {}:{}", host, port);
}

#[derive(Debug)]
enum Response { Ok(i32, String, f64), Err(String) }

fn get_val(r: &Response) -> Option<i32> {
    match r {
        Response::Ok(v, _, _) => Some(*v),
        Response::Err(_)      => None,
    }
}

fn main() {
    let cfg = Config { host:"localhost".into(), port:8080, timeout:30.0, debug:true };
    connect(&cfg);

    for r in [Response::Ok(42,"msg".into(),1.0), Response::Err("oops".into())] {
        match get_val(&r) {
            Some(v) => println!("Got: {}", v),
            None    => println!("Error"),
        }
    }

    // _ in function param
    fn always_zero(_: i32) -> i32 { 0 }
    println!("zeros: {:?}", (0..5).map(always_zero).collect::<Vec<_>>());

    // .. in tuple
    let (first, .., last) = (1, 2, 3, 4, 5);
    println!("first={} last={}", first, last);

    // _x: bind but suppress warning
    let _unused = String::from("won't warn");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_get_val() {
        assert_eq!(get_val(&Response::Ok(7,"x".into(),0.0)), Some(7));
        assert_eq!(get_val(&Response::Err("e".into())), None);
    }
    #[test] fn tuple_wildcard() { let (f,..,l) = (1,2,3,4,5); assert_eq!((f,l),(1,5)); }
}
