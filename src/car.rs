use crate::traits::*;
use gpio_cdev::Chip;
use std::fmt;
use crate::direction::Direction;
use crate::dc_motor::DCMotor;

pub struct Car {
    chip: Chip,

    direction: Direction,

    motors: Vec<DCMotor>
}

impl Car {
    pub fn new(chip:Chip, motors: Vec<DCMotor>) -> Car {
        Car{chip: chip, direction: Direction::None, motors: motors}
    }
}

impl Vehicle for Car {
    fn change_direction(&mut self, dir: Direction) {
        self.direction = dir;
    }

    fn drive(&mut self) {
        for motor in self.motors.iter_mut() {
            motor.rotate(self.direction).unwrap()
        }
    }

    fn stop(&mut self) {
        self.direction = Direction::Stop;
    }
}

impl Smart for Car{}

impl  fmt::Display for Car  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chip: {:?} Motor num: {})", self.chip, self.motors.len())
    }
}

impl  fmt::Debug for Car  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chip: {:?}\nMotors: {:?})", self.chip, self.motors)
    }
}
