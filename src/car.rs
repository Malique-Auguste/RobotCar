use crate::traits::*;
use gpio_cdev::Chip;
use std::fmt;
use crate::direction::Direction;

pub struct Car<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug> {
    id: (char, u32, u32),
    chip: Chip,

    direction: Direction,

    sensors: Vec<A>,
    motors: Vec<B>
}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug> Car<A, B> {
    pub fn new(id: (char, u32, u32), chip:Chip, sensors: Vec<A>, motors: Vec<B>) -> Car<A, B> {
        Car{id: id, chip: chip, direction: Direction::None, sensors: sensors, motors: motors}
    }
}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug> Vehicle for Car<A, B> {
    fn change_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    fn drive(&self) {
        unimplemented!();
    }

    fn stop(&mut self) {
        self.direction = Direction::Stop;
    }
}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug> Smart for Car<A, B>{}

impl<A: Sensor + fmt::Debug, B: Motor+ fmt::Debug>  fmt::Display for Car<A, B>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chip: {:?} Sensor num: {}, Motor num: {})", self.chip, self.sensors.len(), self.motors.len())
    }
}

impl<A: Sensor + fmt::Debug, B: Motor + fmt::Debug>  fmt::Debug for Car<A, B>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chip: {:?}\nSensors: {:?}\nMotors: {:?})", self.chip, self.sensors, self.motors)
    }
}
