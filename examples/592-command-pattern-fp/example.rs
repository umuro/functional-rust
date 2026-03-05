#[derive(Debug,Clone)]
enum DrawCmd {
    MoveTo(f64, f64),
    LineTo(f64, f64),
    SetColor(String),
    SetWidth(f64),
}

#[derive(Debug,Default,Clone)]
struct DrawState {
    x: f64, y: f64,
    color: String,
    width: f64,
}

fn execute(state: &mut DrawState, cmd: &DrawCmd, log: &mut Vec<String>) {
    match cmd {
        DrawCmd::MoveTo(x,y)   => { state.x=*x; state.y=*y; }
        DrawCmd::LineTo(x,y)   => {
            log.push(format!("line ({:.1},{:.1})->({:.1},{:.1}) color={} w={}", state.x,state.y,x,y,state.color,state.width));
            state.x=*x; state.y=*y;
        }
        DrawCmd::SetColor(c)   => { state.color=c.clone(); }
        DrawCmd::SetWidth(w)   => { state.width=*w; }
    }
}

// Command macro: build sequences
fn rect(x: f64, y: f64, w: f64, h: f64) -> Vec<DrawCmd> {
    vec![
        DrawCmd::MoveTo(x,y),
        DrawCmd::LineTo(x+w,y),
        DrawCmd::LineTo(x+w,y+h),
        DrawCmd::LineTo(x,y+h),
        DrawCmd::LineTo(x,y),
    ]
}

fn replay(cmds: &[DrawCmd]) -> (DrawState, Vec<String>) {
    let mut state = DrawState { color:"black".into(), width:1.0, ..Default::default() };
    let mut log = Vec::new();
    for cmd in cmds { execute(&mut state, cmd, &mut log); }
    (state, log)
}

fn main() {
    let mut cmds = vec![DrawCmd::SetColor("red".into()), DrawCmd::SetWidth(2.0)];
    cmds.extend(rect(0.0, 0.0, 10.0, 10.0));
    let (final_state, log) = replay(&cmds);
    for l in &log { println!("{}", l); }
    println!("final pos: ({:.1},{:.1})", final_state.x, final_state.y);
    println!("total commands: {}", cmds.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn rect_cmds() { assert_eq!(rect(0.0,0.0,1.0,1.0).len(), 5); }
    #[test] fn replay_moves() {
        let cmds = vec![DrawCmd::MoveTo(3.0,4.0), DrawCmd::SetColor("blue".into())];
        let (s,_) = replay(&cmds);
        assert_eq!((s.x,s.y), (3.0,4.0));
    }
}
