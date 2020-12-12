use crate::traits::Controller;
use crate::direction::Direction;
use std::io;
use std::fmt;

pub struct CarController {
    last_direction: Direction,
    current_direction: Direction
}

impl CarController {
    pub fn new() -> CarController {
        CarController {last_direction: Direction::None, current_direction: Direction::None}
    }
}

impl Controller for CarController {
    type SignalType = Direction;

    fn set_signal(&mut self) {
        let mut dir = String::new();
        match io::stdin().read_line(&mut dir) {
            Err(_) => return,
            _ => ()
        }

        self.current_direction = Direction::from_str(dir.trim());
    }

    fn get_signal(&self) -> Direction {
        self.current_direction
    }
}

impl fmt::Display for CarController  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Last Direction: {}, Current Direction: {}", self.last_direction, self.last_direction)
    }
}

impl fmt::Debug for CarController  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Last Direction: {:?},\nCurrent Direction: {:?}", self.last_direction, self.current_direction)
    }
}