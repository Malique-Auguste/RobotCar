use crate::direction::Direction;

pub trait Vehicle {
    fn change_direction(&mut self, dir: Direction);
    fn drive(&self);
    fn stop(&mut self);
}

pub trait Smart {  
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
    type MoveData;

    fn rotate(&mut self, data:Self::MoveData) -> Result<(), gpio_cdev::Error>;
}
