#![allow(clippy::all)]
// 1001: Event Loop — Tokio version
// Tokio IS an event loop — demonstrate async event dispatch

use std::collections::VecDeque;
use tokio::sync::mpsc;

/// Event enum
#[derive(Debug, Clone, PartialEq)]
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Timer(String),
    NetworkData(String),
    Quit,
}

/// Application state
#[derive(Debug, Clone, PartialEq)]
struct AppState {
    clicks: u32,
    keys: String,
    timers: u32,
    network_msgs: Vec<String>,
}

impl AppState {
    fn new() -> Self {
        AppState { clicks: 0, keys: String::new(), timers: 0, network_msgs: Vec::new() }
    }
}

/// Pure functional dispatch
fn dispatch(state: AppState, event: &Event) -> AppState {
    match event {
        Event::Click { .. } => AppState { clicks: state.clicks + 1, ..state },
        Event::KeyPress(c) => AppState { keys: format!("{}{}", state.keys, c), ..state },
        Event::Timer(_) => AppState { timers: state.timers + 1, ..state },
        Event::NetworkData(msg) => {
            let mut msgs = state.network_msgs.clone();
            msgs.push(msg.clone());
            AppState { network_msgs: msgs, ..state }
        }
        Event::Quit => state,
    }
}

/// Functional event loop over a Vec
fn run_until_quit(events: Vec<Event>, mut state: AppState) -> AppState {
    for event in events {
        match event {
            Event::Quit => break,
            e => state = dispatch(state, &e),
        }
    }
    state
}

/// Async event loop using tokio mpsc channel
async fn async_event_loop(events: Vec<Event>) -> AppState {
    let (tx, mut rx) = mpsc::channel::<Event>(32);

    // Event producer
    tokio::spawn(async move {
        for e in events {
            tx.send(e).await.unwrap();
        }
    });

    // Event consumer (the "event loop")
    let mut state = AppState::new();
    while let Some(event) = rx.recv().await {
        match event {
            Event::Quit => break,
            e => state = dispatch(state, &e),
        }
    }
    state
}

/// Event loop with multiple async event sources
async fn multi_source_event_loop() -> AppState {
    let (tx, mut rx) = mpsc::channel::<Event>(32);

    // Simulate multiple event sources
    let tx1 = tx.clone();
    tokio::spawn(async move {
        tx1.send(Event::Click { x: 10, y: 20 }).await.ok();
        tx1.send(Event::Click { x: 5, y: 5 }).await.ok();
    });

    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx2.send(Event::KeyPress('h')).await.ok();
        tx2.send(Event::KeyPress('i')).await.ok();
    });

    let tx3 = tx.clone();
    tokio::spawn(async move {
        tx3.send(Event::NetworkData("hello".to_string())).await.ok();
    });

    // Send quit after a short delay
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        tx.send(Event::Quit).await.ok();
    });

    let mut state = AppState::new();
    while let Some(event) = rx.recv().await {
        match event {
            Event::Quit => break,
            e => state = dispatch(state, &e),
        }
    }
    state
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

    #[tokio::test]
    async fn test_async_event_loop() {
        let state = async_event_loop(test_events()).await;
        assert_eq!(state.clicks, 2);
        assert_eq!(state.keys, "hi");
        assert_eq!(state.timers, 2);
        assert_eq!(state.network_msgs.len(), 2);
    }

    #[tokio::test]
    async fn test_quit_stops_processing() {
        let events = vec![
            Event::Click { x: 0, y: 0 },
            Event::Quit,
            Event::Click { x: 0, y: 0 },
        ];
        let state = async_event_loop(events).await;
        assert_eq!(state.clicks, 1);
    }

    #[test]
    fn test_run_until_quit() {
        let state = run_until_quit(test_events(), AppState::new());
        assert_eq!(state.clicks, 2);
        assert_eq!(state.keys, "hi");
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

    #[tokio::test]
    async fn test_empty_events() {
        let state = async_event_loop(vec![]).await;
        assert_eq!(state, AppState::new());
    }

    #[tokio::test]
    async fn test_multi_source() {
        let state = multi_source_event_loop().await;
        // All sources contribute (exact counts depend on scheduling)
        assert!(state.clicks + state.keys.len() as u32 + state.network_msgs.len() as u32 > 0);
    }
}
