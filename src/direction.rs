use std::fmt;
use termion::event::Key;
use std::slice::Iter;

#[derive(Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,

    None,
    Stop
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 6] = [Direction::Forward, Direction::Backward, Direction::Left, Direction::Right, Direction::None, Direction::Stop];
        DIRECTIONS.iter()
    }
}

impl From<Key> for Direction {
    fn from(o: Key) -> Direction {
        match o {
            Key::Up => Direction::Forward,
            Key::Down => Direction::Backward,
            Key::Left => Direction::Left,
            Key::Right => Direction::Right,

            Key::Backspace => Direction::Stop,
            _ => Direction::None,
        }
    }
}

impl fmt::Display for Direction  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Forward => write!(f, "F"),
            Direction::Backward => write!(f, "B"),
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),

            Direction::None => write!(f, "N"),
            Direction::Stop => write!(f, "S")
        }
    }
}

impl fmt::Debug for Direction  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Forward => write!(f, "Forward"),
            Direction::Backward => write!(f, "Backward"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),

            Direction::None => write!(f, "None"),
            Direction::Stop => write!(f, "Stop")
        }
    }
}
