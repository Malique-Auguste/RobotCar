use crate::traits::*;
use gpio_cdev::Chip;
use std::fmt;


pub struct Car<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug>{
    id: (char, u32, u32),
    chip: Chip,
    sensors: Vec<A>,
    motors: Vec<B>
}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug> Car<A, B>{
    pub fn new(id: (char, u32, u32), chip:Chip, sensors: Vec<A>, motors: Vec<B>) -> Car<A, B> {
        Car{id: id, chip: chip, sensors: sensors, motors: motors}
    }
}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug> Robot for Car<A, B>{}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug> Identifiable for Car<A, B> {
    fn get_id(&self) -> (char, u32, u32) {
        self.id
    }

    fn set_id(&mut self, group: char, model: u32, num: u32) {
        self.id = (group, model, num);
    }
}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug>  fmt::Display for Car<A, B>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} : {:?}, {}, {})", self.get_id(), self.chip, self.sensors.len(), self.motors.len())
    }
}

impl<A: Sensor + fmt::Debug, B: Motor + fmt::Debug>  fmt::Debug for Car<A, B>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} :\n{:?}\n{:?},\n{:?})", self.get_id(), self.chip, self.sensors, self.motors)
    }
}
