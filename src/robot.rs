use crate::traits::*;
use gpio_cdev::Chip;
use std::fmt;


pub struct Robot<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug>{
    id: (char, u32, u32),
    chip: Chip,
    sensors: Vec<A>,
    motors: Vec<B>
}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug> Robot<A, B>{
    pub fn new(id: (char, u32, u32), chip:Chip, sensors: Vec<A>, motors: Vec<B>) -> Robot<A, B> {
        Robot{id: id, chip: chip, sensors: sensors, motors: motors}
    }
}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug> Identifiable for Robot<A, B> {
    fn get_id(&self) -> (char, u32, u32) {
        self.id
    }

    fn set_id(&mut self, group: char, model: u32, num: u32) {
        self.id = (group, model, num);
    }
}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug>  fmt::Display for Robot<A, B>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} : {:?}, {}, {})", self.get_id(), self.chip, self.sensors.len(), self.motors.len())
    }
}

impl<A: Sensor + fmt::Debug, B: Motor + fmt::Debug>  fmt::Debug for Robot<A, B>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} :\n{:?}\n{:?},\n{:?})", self.get_id(), self.chip, self.sensors, self.motors)
    }
}
