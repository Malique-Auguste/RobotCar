use gpio_cdev::{Chip, Line, LineHandle, LineRequestFlags, Error};
use std::fmt;
use crate::direction::Direction;
use crate::traits::Motor;

pub struct DCMotor {
    direction: Direction,
    
    motor1: (LineHandle, LineHandle),
    m1_enable: LineHandle,
    
    motor2: Option<(LineHandle, LineHandle)>,
    m2_enable: Option<LineHandle>
}

impl DCMotor {
    pub fn new(chip: &mut Chip, motor1: (u32, u32, u32), motor2: Option<(u32, u32, u32)>) -> Result<DCMotor, Error> {
        let m1_enable = match chip.get_line(motor1.0) {
            Ok(enable_line) => match enable_line.request(LineRequestFlags::OUTPUT, 1, "motor 1 enable line") {
                Ok(handle) => handle,
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e)
        };
        
        let m1_0= match chip.get_line(motor1.1) {
            Ok(motor_line) => match motor_line.request(LineRequestFlags::OUTPUT, 0, "motor 1.0 line") {
                Ok(handle) => handle,
                Err(e) => return Err(e)
            },
            Err(e) => return Err(e)
        };
        
        let m1_1 = match chip.get_line(motor1.2) {
            Ok(motor_line) => match motor_line.request(LineRequestFlags::OUTPUT, 0, "motor 1.1 line") {
                Ok(handle) => handle,
                Err(e) => return Err(e)
            },
            Err(e) => return Err(e)
        };

        if let Some(temp) = motor2 {
            let m2_enable = match chip.get_line(temp.0) {
                Ok(enable_line) => match enable_line.request(LineRequestFlags::OUTPUT, 1, "motor 2 enable line") {
                    Ok(handle) => handle,
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(e)
            };
            
            let m2_0 = match chip.get_line(temp.1) {
                Ok(motor_line) => match motor_line.request(LineRequestFlags::OUTPUT, 0, "motor 2.0 line") {
                    Ok(handle) => handle,
                    Err(e) => return Err(e)
                },
                Err(e) => return Err(e)
            };
            
            let m2_1 = match chip.get_line(temp.2) {
                Ok(motor_line) => match motor_line.request(LineRequestFlags::OUTPUT, 0, "motor 2.1 line") {
                    Ok(handle) => handle,
                    Err(e) => return Err(e)
                },
                Err(e) => return Err(e)
            };

            
            return Ok(DCMotor{direction:Direction::Forward, motor1: (m1_0, m1_1), m1_enable: m1_enable, motor2: Some((m2_0, m2_1)), m2_enable: Some(m2_enable)});
        }

        else {
            return Ok(DCMotor{direction:Direction::Forward, motor1: (m1_0, m1_1), m1_enable: m1_enable, motor2: None, m2_enable: None});
        }
    }
}

impl Motor for DCMotor {
    type MoveData = Direction;

    fn rotate(&mut self, data: Direction) -> Result<(), Error>{
        self.direction = data;

        match self.direction {
            Direction::Forward | Direction::Left => {
                if let Err(e) = self.motor1.0.set_value(0) { return Err(e); }
                if let Err(e) = self.motor1.1.set_value(1) { return Err(e); }
            },

            Direction::Backward | Direction::Right => {
                if let Err(e) = self.motor1.0.set_value(1) { return Err(e); }
                if let Err(e) = self.motor1.1.set_value(0) { return Err(e); }
            },

            Direction::Stop => {
                if let Err(e) = self.motor1.0.set_value(0) { return Err(e); }
                if let Err(e) = self.motor1.1.set_value(0) { return Err(e); }
            },

            _ => unimplemented!()
        }

        if let Some(ref temp) = self.motor2 {

            match self.direction {
                Direction::Forward | Direction::Right => {
                    if let Err(e) = temp.0.set_value(0) { return Err(e); }
                    if let Err(e) = temp.1.set_value(1) { return Err(e); }
                },
    
                Direction::Backward | Direction::Left => {
                    if let Err(e) = temp.0.set_value(1) { return Err(e); }
                    if let Err(e) = temp.1.set_value(0) { return Err(e); }
                },
                
                Direction::Stop => {
                    if let Err(e) = temp.0.set_value(0) { return Err(e); }
                    if let Err(e) = temp.1.set_value(0) { return Err(e); }
                },
                _ => unimplemented!()
            }
        }
        Ok(())
    }
}

impl fmt::Display for DCMotor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.motor2 {
            Some(_) => write!(f, "Direction: {}, Is Single Motor", self.direction),
            None => write!(f, "Direction: {}, Is Duo Motor", self.direction),
        }
        
    }
}

impl fmt::Debug for DCMotor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Direction: {:?}, Motor1: {:?}\n Motor2: {:?}", self.direction, self.motor1, self.motor2)
    }
}
