use crate::traits::Controller;
use crate::direction::Direction;
use std::io;
use std::fmt;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

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
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();

        for c in stdin.keys() {
            self.current_direction = Direction::from(c.unwrap());
            break
        }
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