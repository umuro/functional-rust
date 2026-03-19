#![allow(clippy::all)]
//! .. and _ Wildcards
//!
//! Ignoring parts of a pattern.

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Ignore with _.
pub fn get_x(p: &Point) -> i32 {
    match p {
        Point { x, y: _, z: _ } => *x,
    }
}

/// Ignore multiple with ...
pub fn get_x_short(p: &Point) -> i32 {
    let Point { x, .. } = p;
    *x
}

/// Ignore in tuple.
pub fn first_of_four((a, _, _, _): (i32, i32, i32, i32)) -> i32 {
    a
}

/// Ignore middle elements.
pub fn ends((first, .., last): (i32, i32, i32, i32, i32)) -> (i32, i32) {
    (first, last)
}

/// Wildcard in match.
pub fn is_special(n: i32) -> bool {
    match n {
        0 | 42 | 100 => true,
        _ => false,
    }
}

/// Ignore enum data.
#[derive(Debug)]
pub enum Event {
    Click(i32, i32),
    Scroll(f32),
    KeyPress(char),
}

pub fn is_click(e: &Event) -> bool {
    matches!(e, Event::Click(..))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_x() {
        let p = Point { x: 1, y: 2, z: 3 };
        assert_eq!(get_x(&p), 1);
        assert_eq!(get_x_short(&p), 1);
    }

    #[test]
    fn test_first_of_four() {
        assert_eq!(first_of_four((1, 2, 3, 4)), 1);
    }

    #[test]
    fn test_ends() {
        assert_eq!(ends((1, 2, 3, 4, 5)), (1, 5));
    }

    #[test]
    fn test_is_special() {
        assert!(is_special(42));
        assert!(!is_special(7));
    }

    #[test]
    fn test_is_click() {
        assert!(is_click(&Event::Click(0, 0)));
        assert!(!is_click(&Event::KeyPress('a')));
    }
}
