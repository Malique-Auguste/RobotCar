use gpio_cdev::{Chip, Line, LineRequestFlags, Error};
use std::fmt;
use crate::direction::Direction;
use crate::traits::Motor;

pub struct DCMotor {
    direction: Direction,
    motor1: (Line, Line),
    motor2: Option<(Line, Line)>
}

impl DCMotor {
    pub fn new(chip: &mut Chip, motor1: (u32, u32), motor2: Option<(u32, u32)>) -> Result<DCMotor, Error> {
        let motor1 = match chip.get_line(motor1.0) {
            Ok(m0) => match chip.get_line(motor1.1) {
                Ok(m1) => (m0, m1),
                Err(e) => return Err(e)
            },
            Err(e) => return Err(e)
        };

        if let Some(temp) = motor2 {
            let motor2 = match chip.get_line(temp.0) {
                Ok(m0) => match chip.get_line(temp.1) {
                    Ok(m1) => Some((m0, m1)),
                    Err(e) => return Err(e)
                },
                Err(e) => return Err(e)
            };

            return Ok(DCMotor{direction:Direction::Forward, motor1: motor1, motor2: motor2});
        }

        else {
            return Ok(DCMotor{direction:Direction::Forward, motor1: motor1, motor2: None});
        }
    }
}

impl Motor for DCMotor {
    type MoveData = Direction;

    fn rotate(&mut self, data: Direction) -> Result<(), Error>{
        self.direction = data;

        if let Direction::None = self.direction { return Ok(()); }

        let motor1_0 = match self.motor1.0.request(LineRequestFlags::OUTPUT, 0, "motor 1.0"){
            Ok(val) => val,
            Err(e) => return Err(e)
        };

        let motor1_1 = match self.motor1.1.request(LineRequestFlags::OUTPUT, 0, "motor 1.1") {
            Ok(val) => val,
            Err(e) => return Err(e)
        };

        match self.direction {
            Direction::Forward | Direction::Left => {
                if let Err(e) = motor1_0.set_value(0) { return Err(e); }
                if let Err(e) = motor1_1.set_value(1) { return Err(e); }
            },

            Direction::Backward | Direction::Right => {
                if let Err(e) = motor1_0.set_value(1) { return Err(e); }
                if let Err(e) = motor1_1.set_value(0) { return Err(e); }
            },

            Direction::Stop => {
                if let Err(e) = motor1_0.set_value(0) { return Err(e); }
                if let Err(e) = motor1_1.set_value(0) { return Err(e); }
            },

            _ => unimplemented!()
        }

        if let Some(ref temp) = self.motor2 {
            let motor2_0 = match temp.0.request(LineRequestFlags::OUTPUT, 0, "motor 2.0"){
                Ok(val) => val,
                Err(e) => return Err(e)
            };

            let motor2_1 = match temp.1.request(LineRequestFlags::OUTPUT, 0, "motor 2.1"){
                Ok(val) => val,
                Err(e) => return Err(e)
            };

            match self.direction {
                Direction::Forward | Direction::Right => {
                    if let Err(e) = motor2_0.set_value(0) { return Err(e); }
                    if let Err(e) = motor2_1.set_value(1) { return Err(e); }
                },
    
                Direction::Backward | Direction::Left => {
                    if let Err(e) = motor2_0.set_value(1) { return Err(e); }
                    if let Err(e) = motor2_1.set_value(0) { return Err(e); }
                },
                
                Direction::Stop => {
                    if let Err(e) = motor1_0.set_value(0) { return Err(e); }
                    if let Err(e) = motor1_1.set_value(0) { return Err(e); }
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