use crate::traits::{Controller, Identifiable};
use crate::direction::Direction;
use std::io;
use std::fmt;

pub struct CarController {
    id: (char, u32, u32),
    last_direction: Direction,
    current_direction: Direction
}

impl CarController {
    pub fn new(group: char, model: u32, num: u32) -> CarController {
        CarController {id: (group, model, num), last_direction: Direction::None, current_direction: Direction::None}
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

impl Identifiable for CarController {
    fn get_id(&self) -> (char, u32, u32) {
        self.id
    }

    fn set_id(&mut self, group: char, model: u32, num: u32) {
        self.id = (group, model, num);
    }
}

impl fmt::Display for CarController  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.get_id())
    }
}

impl fmt::Debug for CarController  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} :\n{},\n{}", self.get_id(), self.last_direction, self.current_direction)
    }
}