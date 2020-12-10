pub trait Identifiable {
    //the character is the group example: motor, sensor, robot
    //1st number is the model
    //2nd number is the that devices creation number
    fn get_id(&self) -> (char, u32, u32);
    fn set_id(&mut self, group: char, model: u32, num: u32);
}

pub trait Robot {  
}

pub trait Controller {
    type SignalType;

    fn set_signal(&mut self);
    fn get_signal(&self) -> Self::SignalType;
}

pub trait Sensor { 
    type OkType;
    type ErrorType;

    fn sense(&self) -> Result<Self::OkType, Self::ErrorType>; 
}

pub trait Motor {
}
