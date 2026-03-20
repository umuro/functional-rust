#![allow(clippy::all)]
//! Enum Dispatch Macro
//!
//! Generating match arms for enums.

/// Define enum with dispatch method.
#[macro_export]
macro_rules! dispatch_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name { $($variant,)* }

        impl $name {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant),)*
                }
            }
        }
    };
}

dispatch_enum!(Action {
    Start,
    Stop,
    Pause,
    Resume
});

/// Command pattern with enum.
pub enum Command {
    Print(String),
    Add(i32, i32),
    Exit,
}

impl Command {
    pub fn execute(&self) -> String {
        match self {
            Command::Print(s) => format!("Print: {}", s),
            Command::Add(a, b) => format!("Add: {} + {} = {}", a, b, a + b),
            Command::Exit => "Exit".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_name() {
        assert_eq!(Action::Start.name(), "Start");
        assert_eq!(Action::Stop.name(), "Stop");
    }

    #[test]
    fn test_command_print() {
        let c = Command::Print("Hello".into());
        assert!(c.execute().contains("Hello"));
    }

    #[test]
    fn test_command_add() {
        let c = Command::Add(2, 3);
        assert!(c.execute().contains("5"));
    }

    #[test]
    fn test_command_exit() {
        let c = Command::Exit;
        assert_eq!(c.execute(), "Exit");
    }

    #[test]
    fn test_all_actions() {
        let actions = [Action::Start, Action::Stop, Action::Pause, Action::Resume];
        assert_eq!(actions.len(), 4);
    }
}
