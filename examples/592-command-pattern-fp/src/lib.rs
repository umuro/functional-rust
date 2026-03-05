//! # Command Pattern (Functional Style)
//!
//! Commands as data structures that can be stored, replayed, and undone.

/// Drawing commands for a simple graphics system.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawCmd {
    MoveTo(f64, f64),
    LineTo(f64, f64),
    ArcTo(f64, f64, f64), // x, y, radius
    SetColor(String),
    SetWidth(f64),
    Fill,
    Stroke,
}

/// State of the drawing context.
#[derive(Debug, Clone, Default)]
pub struct DrawState {
    pub x: f64,
    pub y: f64,
    pub color: String,
    pub width: f64,
}

impl DrawState {
    pub fn new() -> Self {
        Self {
            color: "black".into(),
            width: 1.0,
            ..Default::default()
        }
    }
}

/// Execute a single command, updating state and optionally producing output.
pub fn execute(state: &mut DrawState, cmd: &DrawCmd) -> Option<String> {
    match cmd {
        DrawCmd::MoveTo(x, y) => {
            state.x = *x;
            state.y = *y;
            None
        }
        DrawCmd::LineTo(x, y) => {
            let log = format!(
                "line ({:.1},{:.1})->({:.1},{:.1}) color={} width={}",
                state.x, state.y, x, y, state.color, state.width
            );
            state.x = *x;
            state.y = *y;
            Some(log)
        }
        DrawCmd::ArcTo(x, y, r) => {
            let log = format!(
                "arc ({:.1},{:.1})->({:.1},{:.1}) r={:.1}",
                state.x, state.y, x, y, r
            );
            state.x = *x;
            state.y = *y;
            Some(log)
        }
        DrawCmd::SetColor(c) => {
            state.color = c.clone();
            None
        }
        DrawCmd::SetWidth(w) => {
            state.width = *w;
            None
        }
        DrawCmd::Fill => Some(format!("fill with {}", state.color)),
        DrawCmd::Stroke => Some(format!("stroke with {} width={}", state.color, state.width)),
    }
}

/// Build a rectangle as a sequence of commands.
pub fn rect(x: f64, y: f64, w: f64, h: f64) -> Vec<DrawCmd> {
    vec![
        DrawCmd::MoveTo(x, y),
        DrawCmd::LineTo(x + w, y),
        DrawCmd::LineTo(x + w, y + h),
        DrawCmd::LineTo(x, y + h),
        DrawCmd::LineTo(x, y),
    ]
}

/// Build a circle approximation.
pub fn circle(cx: f64, cy: f64, r: f64) -> Vec<DrawCmd> {
    vec![
        DrawCmd::MoveTo(cx + r, cy),
        DrawCmd::ArcTo(cx, cy + r, r),
        DrawCmd::ArcTo(cx - r, cy, r),
        DrawCmd::ArcTo(cx, cy - r, r),
        DrawCmd::ArcTo(cx + r, cy, r),
    ]
}

/// Replay a sequence of commands.
pub fn replay(cmds: &[DrawCmd]) -> (DrawState, Vec<String>) {
    let mut state = DrawState::new();
    let mut log = Vec::new();
    for cmd in cmds {
        if let Some(entry) = execute(&mut state, cmd) {
            log.push(entry);
        }
    }
    (state, log)
}

/// Pure command application (returns new state, no mutation).
pub fn apply(state: DrawState, cmd: &DrawCmd) -> DrawState {
    let mut new_state = state;
    execute(&mut new_state, cmd);
    new_state
}

/// Count different command types.
pub fn count_commands(cmds: &[DrawCmd]) -> (usize, usize, usize) {
    let moves = cmds.iter().filter(|c| matches!(c, DrawCmd::MoveTo(_, _))).count();
    let lines = cmds.iter().filter(|c| matches!(c, DrawCmd::LineTo(_, _))).count();
    let style = cmds.iter().filter(|c| matches!(c, DrawCmd::SetColor(_) | DrawCmd::SetWidth(_))).count();
    (moves, lines, style)
}

/// Optimize commands by removing redundant style changes.
pub fn optimize(cmds: Vec<DrawCmd>) -> Vec<DrawCmd> {
    let mut result = Vec::new();
    let mut last_color: Option<String> = None;
    let mut last_width: Option<f64> = None;

    for cmd in cmds {
        match &cmd {
            DrawCmd::SetColor(c) if last_color.as_ref() == Some(c) => continue,
            DrawCmd::SetWidth(w) if last_width == Some(*w) => continue,
            DrawCmd::SetColor(c) => last_color = Some(c.clone()),
            DrawCmd::SetWidth(w) => last_width = Some(*w),
            _ => {}
        }
        result.push(cmd);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_to() {
        let mut state = DrawState::new();
        execute(&mut state, &DrawCmd::MoveTo(10.0, 20.0));
        assert_eq!((state.x, state.y), (10.0, 20.0));
    }

    #[test]
    fn test_line_to() {
        let mut state = DrawState::new();
        state.x = 5.0;
        state.y = 5.0;
        let log = execute(&mut state, &DrawCmd::LineTo(10.0, 10.0));
        assert!(log.is_some());
        assert_eq!((state.x, state.y), (10.0, 10.0));
    }

    #[test]
    fn test_rect_commands() {
        let cmds = rect(0.0, 0.0, 10.0, 10.0);
        assert_eq!(cmds.len(), 5);
    }

    #[test]
    fn test_replay() {
        let cmds = vec![
            DrawCmd::SetColor("red".into()),
            DrawCmd::MoveTo(0.0, 0.0),
            DrawCmd::LineTo(10.0, 0.0),
            DrawCmd::LineTo(10.0, 10.0),
        ];
        let (state, log) = replay(&cmds);
        assert_eq!(state.color, "red");
        assert_eq!(log.len(), 2); // Two LineTo commands produce output
    }

    #[test]
    fn test_pure_apply() {
        let state = DrawState::new();
        let state2 = apply(state.clone(), &DrawCmd::MoveTo(5.0, 5.0));
        assert_eq!(state.x, 0.0); // Original unchanged
        assert_eq!(state2.x, 5.0);
    }

    #[test]
    fn test_count_commands() {
        let cmds = vec![
            DrawCmd::MoveTo(0.0, 0.0),
            DrawCmd::LineTo(1.0, 1.0),
            DrawCmd::LineTo(2.0, 2.0),
            DrawCmd::SetColor("red".into()),
        ];
        let (moves, lines, style) = count_commands(&cmds);
        assert_eq!(moves, 1);
        assert_eq!(lines, 2);
        assert_eq!(style, 1);
    }

    #[test]
    fn test_optimize_removes_redundant() {
        let cmds = vec![
            DrawCmd::SetColor("red".into()),
            DrawCmd::SetColor("red".into()), // Redundant
            DrawCmd::LineTo(1.0, 1.0),
            DrawCmd::SetColor("blue".into()),
        ];
        let optimized = optimize(cmds);
        assert_eq!(optimized.len(), 3);
    }

    #[test]
    fn test_circle() {
        let cmds = circle(0.0, 0.0, 5.0);
        assert_eq!(cmds.len(), 5); // MoveTo + 4 ArcTo
    }
}
