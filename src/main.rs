mod car;
mod car_control;
mod direction;
mod dc_motor;
mod ultrasound;
mod traits;


use dc_motor::DCMotor;
use direction::Direction;
use traits::*;
use gpio_cdev::{Chip, LineRequestFlags};
use std::time;

fn main() {
    
    for _ in 0..3 {
    println!("\n\nHello, world!");
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    
    let mut motor = DCMotor::new(&mut chip, (25, 23, 24), None).unwrap();
    println!("\nMotor {:?}", motor);

    let t = time::Instant::now();
    println!("\nRotating Forward");
    println!("{:?}", motor.rotate(Direction::Forward));

    while t.elapsed().as_secs_f32() < 3.5 {}
    println!("Rotating Backward");
    println!("{:?}", motor.rotate(Direction::Backward));

    while t.elapsed().as_secs_f32() < 7. {}
    println!("Stopped rotating");
    println!("{:?}", motor.rotate(Direction::Stop));

    
    
}
}
