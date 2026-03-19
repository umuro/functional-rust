#![allow(clippy::all)]
// 1001: Simple Event Loop
// Poll events, dispatch enum handlers, accumulate state

use std::collections::VecDeque;

// --- Event enum ---
#[derive(Debug, Clone, PartialEq)]
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Timer(String),
    NetworkData(String),
    Quit,
}

// --- Application state ---
#[derive(Debug, Clone, PartialEq)]
struct AppState {
    clicks: u32,
    keys: String,
    timers: u32,
    network_msgs: Vec<String>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            clicks: 0,
            keys: String::new(),
            timers: 0,
            network_msgs: Vec::new(),
        }
    }
}

// --- Pure functional dispatch: one event → next state ---
fn dispatch(state: AppState, event: &Event) -> AppState {
    match event {
        Event::Click { .. } => AppState {
            clicks: state.clicks + 1,
            ..state
        },
        Event::KeyPress(c) => AppState {
            keys: format!("{}{}", state.keys, c),
            ..state
        },
        Event::Timer(_) => AppState {
            timers: state.timers + 1,
            ..state
        },
        Event::NetworkData(msg) => {
            let mut msgs = state.network_msgs.clone();
            msgs.push(msg.clone());
            AppState {
                network_msgs: msgs,
                ..state
            }
        }
        Event::Quit => state, // handled by loop
    }
}

// --- Approach 1: Functional event loop over a Vec ---
fn run_event_loop(events: Vec<Event>, init: AppState) -> AppState {
    events.iter().fold(init, |state, event| {
        if event == &Event::Quit {
            state
        }
        // stop processing new events via fold
        else {
            dispatch(state, event)
        }
    })
}

// Better version that actually stops at Quit:
fn run_until_quit(events: Vec<Event>, mut state: AppState) -> AppState {
    for event in events {
        match event {
            Event::Quit => break,
            e => state = dispatch(state, &e),
        }
    }
    state
}

// --- Approach 2: Event loop with a queue (mutable, real-world style) ---
struct EventLoop {
    queue: VecDeque<Event>,
    state: AppState,
}

impl EventLoop {
    fn new(state: AppState) -> Self {
        EventLoop {
            queue: VecDeque::new(),
            state,
        }
    }

    fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }

    fn push_many(&mut self, events: Vec<Event>) {
        for e in events {
            self.queue.push_back(e);
        }
    }

    fn run(&mut self) {
        while let Some(event) = self.queue.pop_front() {
            match event {
                Event::Quit => break,
                e => self.state = dispatch(self.state.clone(), &e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_events() -> Vec<Event> {
        vec![
            Event::Click { x: 10, y: 20 },
            Event::KeyPress('h'),
            Event::KeyPress('i'),
            Event::Timer("heartbeat".to_string()),
            Event::NetworkData("hello".to_string()),
            Event::Click { x: 5, y: 5 },
            Event::NetworkData("world".to_string()),
            Event::Timer("refresh".to_string()),
            Event::Quit,
            Event::Click { x: 0, y: 0 }, // ignored
        ]
    }

    #[test]
    fn test_run_until_quit() {
        let state = run_until_quit(test_events(), AppState::new());
        assert_eq!(state.clicks, 2);
        assert_eq!(state.keys, "hi");
        assert_eq!(state.timers, 2);
        assert_eq!(state.network_msgs.len(), 2);
    }

    #[test]
    fn test_quit_stops_processing() {
        let events = vec![
            Event::Click { x: 0, y: 0 },
            Event::Quit,
            Event::Click { x: 0, y: 0 }, // should not be processed
        ];
        let state = run_until_quit(events, AppState::new());
        assert_eq!(state.clicks, 1);
    }

    #[test]
    fn test_event_loop_queue() {
        let mut el = EventLoop::new(AppState::new());
        el.push_many(test_events());
        el.run();
        assert_eq!(el.state.clicks, 2);
        assert_eq!(el.state.keys, "hi");
    }

    #[test]
    fn test_dispatch_click() {
        let s = dispatch(AppState::new(), &Event::Click { x: 5, y: 5 });
        assert_eq!(s.clicks, 1);
    }

    #[test]
    fn test_dispatch_network() {
        let s = dispatch(AppState::new(), &Event::NetworkData("test".to_string()));
        assert_eq!(s.network_msgs, vec!["test"]);
    }

    #[test]
    fn test_empty_events() {
        let state = run_until_quit(vec![], AppState::new());
        assert_eq!(state, AppState::new());
    }
}
