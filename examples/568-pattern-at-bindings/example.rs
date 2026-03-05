#[derive(Debug)]
enum Event { Click{x:i32,y:i32}, Key(char), Resize(u32,u32) }

fn handle(ev: &Event) -> String {
    match ev {
        e @ Event::Click{x,y} if *x>0 && *y>0 => format!("valid click {:?}", e),
        Event::Click{..}                        => "invalid click".into(),
        Event::Key(c @ 'a'..='z')              => format!("lower: {}", c),
        Event::Key(c @ 'A'..='Z')              => format!("upper: {}", c),
        Event::Key(c)                           => format!("other: {}", c),
        Event::Resize(w,h)                      => format!("resize {}x{}", w, h),
    }
}

fn categorize(n: i32) -> (&'static str, i32) {
    match n {
        x @ 0           => ("zero",           x),
        x @ 1..=100     => ("small positive", x),
        x @ 101..=1000  => ("medium",         x),
        x               => ("large",          x),
    }
}

fn main() {
    for ev in [Event::Click{x:10,y:20}, Event::Click{x:-1,y:5},
               Event::Key('a'), Event::Key('Z'), Event::Resize(800,600)] {
        println!("{}", handle(&ev));
    }
    for n in [0,50,500,5000] {
        let (cat,v) = categorize(n); println!("{}: {}", cat, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn cat_zero()  { assert_eq!(categorize(0).0, "zero"); }
    #[test] fn cat_small() { assert_eq!(categorize(42).0, "small positive"); }
    #[test] fn valid_click() { assert!(handle(&Event::Click{x:1,y:1}).contains("valid")); }
}
