use gpio_cdev::Error;

pub trait Identifiable {
    //the character is the group example: motor, sensor, robot
    //1st number is the model
    //2nd number is the that devices creation number
    fn get_id(&self) -> (char, u32, u32);
    fn set_id(&mut self, group: char, model: u32, num: u32);
}

pub trait Sensor { 
    fn sense(&self) -> Result<f32, Error>; 
}

pub trait Motor {
    fn forward(&self);
    fn backward(&self);
}