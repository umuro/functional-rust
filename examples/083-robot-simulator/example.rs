/// Robot Simulator — State with Immutable Records
///
/// Ownership: Robot is a small Copy type (all fields are Copy).
/// Methods return new robots (functional style) rather than mutating.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction { North, East, South, West }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub dir: Direction,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction { TurnLeft, TurnRight, Advance }

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

impl Robot {
    pub fn new(x: i32, y: i32, dir: Direction) -> Self {
        Robot { x, y, dir }
    }

    /// Returns a new Robot — functional update (like OCaml { r with ... })
    pub fn advance(self) -> Self {
        match self.dir {
            Direction::North => Robot { y: self.y + 1, ..self },
            Direction::East => Robot { x: self.x + 1, ..self },
            Direction::South => Robot { y: self.y - 1, ..self },
            Direction::West => Robot { x: self.x - 1, ..self },
        }
    }

    pub fn execute(self, instr: Instruction) -> Self {
        match instr {
            Instruction::TurnLeft => Robot { dir: self.dir.turn_left(), ..self },
            Instruction::TurnRight => Robot { dir: self.dir.turn_right(), ..self },
            Instruction::Advance => self.advance(),
        }
    }

    /// Run a sequence of instructions — fold pattern
    pub fn run(self, instructions: &[Instruction]) -> Self {
        instructions.iter().fold(self, |r, &i| r.execute(i))
    }
}

/// Version 2: Parse instruction string
impl Robot {
    pub fn run_string(self, s: &str) -> Self {
        s.chars().fold(self, |r, c| match c {
            'L' => r.execute(Instruction::TurnLeft),
            'R' => r.execute(Instruction::TurnRight),
            'A' => r.execute(Instruction::Advance),
            _ => r,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_north() {
        let r = Robot::new(0, 0, Direction::North).advance();
        assert_eq!(r, Robot::new(0, 1, Direction::North));
    }

    #[test]
    fn test_turn_sequence() {
        let r = Robot::new(0, 0, Direction::North);
        let r = r.run(&[Instruction::Advance, Instruction::TurnRight,
                        Instruction::Advance, Instruction::Advance,
                        Instruction::TurnLeft, Instruction::Advance]);
        assert_eq!((r.x, r.y), (2, 2));
    }

    #[test]
    fn test_full_rotation() {
        let r = Robot::new(0, 0, Direction::North);
        let r = r.run(&[Instruction::TurnRight; 4]);
        assert_eq!(r.dir, Direction::North);
    }

    #[test]
    fn test_string_instructions() {
        let r = Robot::new(0, 0, Direction::North).run_string("ARAALA");
        assert_eq!((r.x, r.y), (2, 2));
    }

    #[test]
    fn test_immutability() {
        let r1 = Robot::new(0, 0, Direction::North);
        let r2 = r1.advance(); // r1 is still valid (Copy)
        assert_eq!(r1.y, 0);
        assert_eq!(r2.y, 1);
    }
}

fn main() {
    println!("{:?}", r, Robot::new(0, 1, Direction::North));
    println!("{:?}", (r.x, r.y), (2, 2));
    println!("{:?}", r.dir, Direction::North);
}
