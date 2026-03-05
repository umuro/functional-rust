// Scott-encoded Option<i32>
// None  = |on_none, on_some| on_none()
// Some v = |on_none, on_some| on_some(v)
use std::rc::Rc;

type ScottOption<T, R> = Rc<dyn Fn(Box<dyn Fn() -> R>, Box<dyn Fn(T) -> R>) -> R>;

fn scott_none<T: 'static, R: 'static>() -> ScottOption<T, R> {
    Rc::new(|on_none: Box<dyn Fn() -> R>, _| on_none())
}

fn scott_some<T: Clone + 'static, R: 'static>(v: T) -> ScottOption<T, R> {
    Rc::new(move |_, on_some: Box<dyn Fn(T) -> R>| on_some(v.clone()))
}

fn scott_match<T, R>(
    m: &ScottOption<T, R>,
    on_none: impl Fn() -> R + 'static,
    on_some: impl Fn(T) -> R + 'static,
) -> R {
    m(Box::new(on_none), Box::new(on_some))
}

// Simpler version with concrete type for clarity
type SBool<R> = Box<dyn Fn(R, R) -> R>;

fn s_true<R: Clone + 'static>() -> SBool<R> { Box::new(|t, _f| t) }
fn s_false<R: Clone + 'static>() -> SBool<R> { Box::new(|_t, f| f) }
fn s_if<R: Clone + 'static>(b: SBool<R>, t: R, f: R) -> R { b(t, f) }

fn main() {
    // Scott Option
    let none  = scott_none::<i32,String>();
    let some42 = scott_some::<i32,String>(42);

    let r1 = scott_match(&none,  || "nothing".into(), |v| format!("got {}", v));
    let r2 = scott_match(&some42,|| "nothing".into(), |v| format!("got {}", v));
    println!("none:  {}", r1);
    println!("some42:{}", r2);

    // Scott Bool
    let t: SBool<&str> = s_true();
    let f: SBool<&str> = s_false();
    println!("s_if true  = {}", s_if(t, "yes", "no"));
    println!("s_if false = {}", s_if(f, "yes", "no"));

    // Scott encoding reveals: match IS function application
    println!("Scott encoding: match is just function application over continuations");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn scott_none_test() {
        let n = scott_none::<i32,i32>();
        let r = scott_match(&n, || -1, |v| v);
        assert_eq!(r, -1);
    }
    #[test] fn scott_some_test() {
        let s = scott_some::<i32,i32>(42);
        let r = scott_match(&s, || -1, |v| v);
        assert_eq!(r, 42);
    }
}
