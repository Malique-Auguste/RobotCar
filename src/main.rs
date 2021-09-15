
mod car;
mod car_control;
mod direction;
mod dc_motor;
mod ultrasound;
mod traits;

use car::Car;
use dc_motor::DCMotor;
use direction::Direction;
use traits::*;
use gpio_cdev::{Chip, LineRequestFlags};
use std::time;


fn main() {
    println!("\n\nHello, world!");
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    
    let mut motor = DCMotor::new(&mut chip, (25, 23, 24), Some((11, 9, 10))).unwrap();
    println!("\nMotor {:?}", motor);

    let mut car = Car::new(chip, vec![motor]);

    for dir in Direction::iterator() {
        let t = time::Instant::now();
        println!("\nRotating {:?}", dir);
        car.change_direction(*dir);
        car.drive();
        while t.elapsed().as_secs_f32() < 3. {}     
    }
}


/*
use car_control::CarController;
use traits::Controller;

fn main() {

    let mut cc = CarController::new();
    for _ in 0..6 {
        cc.set_signal();
        println!("dir {:?}", cc.get_signal());
    }
}
*/
