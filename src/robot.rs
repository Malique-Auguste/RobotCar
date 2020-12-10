use crate::traits::*;
use std::fmt;

#[derive(Clone)]
pub struct Robot<A: Sensor, B: Motor>{
    id: (char, u32, u32),
    sensors: Vec<A>,
    motors: Vec<B>
}

impl<A: Sensor, B: Motor> Robot<A, B>{
    pub fn new(id: (char, u32, u32), sensors: Vec<A>, motors: Vec<B>) -> Robot<A, B> {
        Robot{id: id, sensors: sensors, motors: motors}
    }
}

impl<A: Sensor, B: Motor> Identifiable for Robot<A, B> {
    fn get_id(&self) -> (char, u32, u32) {
        self.id
    }

    fn set_id(&mut self, group: char, model: u32, num: u32) {
        self.id = (group, model, num);
    }
}

impl<A: Sensor, B: Motor>  fmt::Display for Robot<A, B>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} : {}, {})", self.get_id(), self.sensors.len(), self.motors.len())
    }
}

impl<A: Sensor, B: Motor>  fmt::Debug for Robot<A, B>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} :\n{:?},\n{:?})", self.get_id(), self.sensors, self.motors)
    }
}