
mod car;
mod car_control;
mod direction;
mod dc_motor;
mod ultrasound;
mod traits;

/*
use dc_motor::DCMotor;
use direction::Direction;
use traits::*;
use gpio_cdev::{Chip};
use std::time;


fn main() {
    println!("Hello, world!");
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    println!("\n{:?}", chip);

    let mut motor = DCMotor::new(&mut chip, (18, 23), None).unwrap();
    println!("\nMotor {:?}", motor);

    let t = time::Instant::now();
    motor.rotate(Direction::Forward);

    while t.elapsed().as_secs_f32() < 5.0 {}
    motor.rotate(Direction::Backward);

    while t.elapsed().as_secs_f32() < 10.0 {}
    motor.rotate(Direction::Stop);
}
*/


use car_control::CarController;
use traits::Controller;

fn main() {

    let mut cc = CarController::new();
    for _ in 0..6 {
        cc.set_signal();
        println!("dir {:?}", cc.get_signal());
    }
}