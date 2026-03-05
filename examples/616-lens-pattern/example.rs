// Type-safe lens implementation
struct Lens<S, A, F: Fn(&S) -> A, G: Fn(A, S) -> S> {
    get_fn: F,
    set_fn: G,
    _phantom: std::marker::PhantomData<(S,A)>,
}

impl<S: Clone, A: Clone, F: Fn(&S)->A, G: Fn(A,S)->S> Lens<S,A,F,G> {
    fn new(get_fn: F, set_fn: G) -> Self {
        Lens { get_fn, set_fn, _phantom: std::marker::PhantomData }
    }
    fn get(&self, s: &S) -> A { (self.get_fn)(s) }
    fn set(&self, a: A, s: S) -> S { (self.set_fn)(a, s) }
    fn over(&self, f: impl Fn(A)->A, s: S) -> S {
        let a = (self.get_fn)(&s);
        (self.set_fn)(f(a), s)
    }
}

// Simple, practical lens using closures stored in structs
#[derive(Clone)]
struct SimpleLens<S, A> {
    getter: Box<dyn Fn(&S) -> A>,
    setter: Box<dyn Fn(A, &S) -> S>,
}

impl<S: Clone, A: Clone> SimpleLens<S, A> {
    fn new(getter: impl Fn(&S)->A+'static, setter: impl Fn(A,&S)->S+'static) -> Self {
        SimpleLens { getter: Box::new(getter), setter: Box::new(setter) }
    }
    fn get(&self, s: &S) -> A { (self.getter)(s) }
    fn set(&self, a: A, s: &S) -> S { (self.setter)(a, s) }
    fn over(&self, f: impl Fn(A)->A, s: &S) -> S { self.set(f(self.get(s)), s) }
    fn compose<B: Clone+'static>(&self, other: SimpleLens<A,B>) -> SimpleLens<S,B> {
        let self_getter  = self.getter.clone();
        let self_setter  = self.setter.clone();
        let other_getter = other.getter.clone();
        let other_setter = other.setter.clone();
        SimpleLens::new(
            move |s| other_getter(&self_getter(s)),
            move |b, s| {
                let a = self_getter(s);
                let new_a = other_setter(b, &a);
                self_setter(new_a, s)
            },
        )
    }
}

impl<S: Clone, A: Clone> Clone for SimpleLens<S, A> {
    fn clone(&self) -> Self {
        // We can't clone Box<dyn Fn>; use Rc for sharing
        panic!("Use Rc<SimpleLens> for cloning")
    }
}

// Practical domain
#[derive(Debug,Clone)]
struct Coords  { lat: f64, lon: f64 }
#[derive(Debug,Clone)]
struct Location { name: String, coords: Coords }
#[derive(Debug,Clone)]
struct Event   { title: String, location: Location, attendees: u32 }

fn make_event() -> Event {
    Event {
        title: "Conf".into(),
        location: Location { name: "Hall A".into(), coords: Coords { lat:42.3, lon:-71.0 } },
        attendees: 100,
    }
}

// Use closures directly for the practical example
fn get_lat(e: &Event) -> f64  { e.location.coords.lat }
fn set_lat(lat: f64, e: &Event) -> Event {
    let mut e2 = e.clone();
    e2.location.coords.lat = lat;
    e2
}

fn main() {
    let e = make_event();
    println!("lat: {:.1}", get_lat(&e));
    let e2 = set_lat(get_lat(&e) + 1.0, &e);
    println!("new lat: {:.1}", get_lat(&e2));

    // Nested update without lens (boilerplate)
    let e3 = Event {
        location: Location {
            coords: Coords { lat: e.location.coords.lat, lon: e.location.coords.lon + 5.0 },
            ..e.location.clone()
        },
        ..e.clone()
    };
    println!("new lon: {:.1}", e3.location.coords.lon);

    // Update attendees
    let e4 = Event { attendees: e.attendees * 2, ..e.clone() };
    println!("attendees*2: {}", e4.attendees);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_get_lat() { let e=make_event(); assert_eq!(get_lat(&e), 42.3); }
    #[test] fn test_set_lat() { let e=make_event(); let e2=set_lat(99.0,&e); assert_eq!(get_lat(&e2), 99.0); }
}
