#[derive(Debug,Clone)]
enum State { Idle, Running(u32), Paused(u32), Done(u32) }

#[derive(Debug,Clone,Copy)]
enum Event { Start, Tick, Pause, Resume, Stop }

fn transition(state: State, event: Event) -> State {
    match (state, event) {
        (State::Idle,       Event::Start)  => State::Running(0),
        (State::Running(n), Event::Tick)   => State::Running(n+1),
        (State::Running(n), Event::Pause)  => State::Paused(n),
        (State::Running(n), Event::Stop)   => State::Done(n),
        (State::Paused(n),  Event::Resume) => State::Running(n),
        (State::Paused(n),  Event::Stop)   => State::Done(n),
        (s,                 _)             => s,
    }
}

fn describe(s: &State) -> String {
    match s {
        State::Idle       => "idle".into(),
        State::Running(n) => format!("running (tick {})", n),
        State::Paused(n)  => format!("paused at {}", n),
        State::Done(n)    => format!("done after {} ticks", n),
    }
}

// Traffic light automaton
#[derive(Debug,Clone,Copy,PartialEq)]
enum Traffic { Red, Green, Yellow }

fn next_traffic(t: Traffic) -> Traffic {
    match t { Traffic::Red=>Traffic::Green, Traffic::Green=>Traffic::Yellow, Traffic::Yellow=>Traffic::Red }
}

fn main() {
    let events = [Event::Start,Event::Tick,Event::Tick,Event::Pause,Event::Resume,Event::Tick,Event::Stop];
    let mut s = State::Idle;
    for e in events { s = transition(s, e); println!("{:?} -> {}", e, describe(&s)); }

    let mut t = Traffic::Red;
    for _ in 0..6 { t = next_traffic(t); print!("{:?} ", t); } println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn start_tick() {
        let s = transition(transition(State::Idle, Event::Start), Event::Tick);
        assert!(matches!(s, State::Running(1)));
    }
    #[test] fn pause_resume() {
        let s = transition(State::Running(5), Event::Pause);
        assert!(matches!(s, State::Paused(5)));
        let s2 = transition(s, Event::Resume);
        assert!(matches!(s2, State::Running(5)));
    }
}
