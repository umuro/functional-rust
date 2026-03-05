// 440. Logging macros pattern (no external crates)

use std::sync::atomic::{AtomicU8, Ordering};

#[repr(u8)] #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Level { Debug=0, Info=1, Warn=2, Error=3 }

impl Level {
    fn as_str(self) -> &'static str {
        match self { Level::Debug=>"DEBUG", Level::Info=>"INFO",
                     Level::Warn=>"WARN",  Level::Error=>"ERROR" }
    }
}

static MIN_LEVEL: AtomicU8 = AtomicU8::new(Level::Info as u8);

pub fn set_level(l: Level) { MIN_LEVEL.store(l as u8, Ordering::Relaxed); }
pub fn is_enabled(l: Level) -> bool { l as u8 >= MIN_LEVEL.load(Ordering::Relaxed) }

pub fn log_impl(level: Level, file: &str, line: u32, msg: &str) {
    if is_enabled(level) {
        eprintln!("[{}] {}:{} — {}", level.as_str(), file, line, msg);
    }
}

#[macro_export] macro_rules! log {
    ($lv:expr, $($a:tt)*) => { $crate::log_impl($lv, file!(), line!(), &format!($($a)*)) };
}
#[macro_export] macro_rules! debug { ($($a:tt)*) => { log!($crate::Level::Debug, $($a)*) }; }
#[macro_export] macro_rules! info  { ($($a:tt)*) => { log!($crate::Level::Info,  $($a)*) }; }
#[macro_export] macro_rules! warn  { ($($a:tt)*) => { log!($crate::Level::Warn,  $($a)*) }; }
#[macro_export] macro_rules! error { ($($a:tt)*) => { log!($crate::Level::Error, $($a)*) }; }

fn main() {
    info!("Starting v{}", "1.0");
    debug!("Hidden at Info level");
    warn!("Low memory: {} MB", 42);
    error!("Fatal: {}", "disk full");
    set_level(Level::Debug);
    debug!("Debug now visible: x={}", 99);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_levels()   { assert!(Level::Debug < Level::Error); }
    #[test] fn test_enabled()  {
        set_level(Level::Warn);
        assert!(!is_enabled(Level::Info));
        assert!(is_enabled(Level::Error));
        set_level(Level::Info); }
    #[test] fn test_str()      { assert_eq!(Level::Warn.as_str(), "WARN"); }
}
