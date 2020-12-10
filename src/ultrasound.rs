use gpio_cdev::{Chip, Line, LineRequestFlags, LineHandle, Error};
use std::{thread, time, fmt};
use crate::traits::{Identifiable, Sensor};

const TRIGGER_DURATION: time::Duration = time::Duration::from_nanos(1);
const HALF_SPEED_OF_SOUND:f32 = 17150.0;

#[derive(Clone)]
pub struct Ultrasound {
    id: (char, u32, u32),
    trigger: Line,
    echo: Line
}

impl Ultrasound {
    pub fn new(id: (char, u32, u32), chip: &mut Chip, trigger_num: u32, echo_num: u32) -> Result<Ultrasound, gpio_cdev::Error> {
        let trigger = match chip.get_line(trigger_num){
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        let echo = match chip.get_line(echo_num){
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        Ok(Ultrasound{id: id, trigger: trigger, echo: echo})
    }
}

impl Sensor for Ultrasound{
    fn sense(&self) -> Result<f32, Error> {
        let trigger = match self.trigger.request(LineRequestFlags::OUTPUT, 0, "triggers sensor") {
            Ok(val) => val,
            Err(e) => return Err(e)
        };

        let echo = match self.echo.request(LineRequestFlags::INPUT, 0, "echo sensor") {
            Ok(val) => val,
            Err(e) => return Err(e)
        };

        println!("Initialising data");
        trigger.set_value(0);
        thread::sleep(time::Duration::from_secs(1));

        println!("Triggering");
        trigger.set_value(1);
        thread::sleep(TRIGGER_DURATION);
        trigger.set_value(0);

        let pulse_start = loop {
            match echo.get_value() {
                Ok(val) => {
                    if val == 1 {
                        break time::Instant::now();
                    }
                }
                Err(e) => return Err(e)
            }
        };

        let pulse_duration = loop {
            match echo.get_value() {
                Ok(val) => {
                    if val == 0 {
                        break pulse_start.elapsed();
                    }
                }
                Err(e) => return Err(e)
            }
        };
        
        Ok(pulse_duration.as_secs_f32() * HALF_SPEED_OF_SOUND)
    }
}

impl Identifiable for Ultrasound {
    fn get_id(&self) -> (char, u32, u32) {
        self.id
    }
    
    fn set_id(&mut self, group: char, model: u32, num: u32) {
        self.id = (group, model, num);
    }
}

impl fmt::Display for Ultrasound  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.get_id())
    }
}

impl fmt::Debug for Ultrasound  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} :\n{:?},\n{:?}", self.get_id(), self.trigger, self.echo)
    }
}
