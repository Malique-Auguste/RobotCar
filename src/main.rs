mod car;
mod car_control;
mod direction;
mod dc_motor;
mod ultrasound;
mod traits;



use car_control::CarController;
use ultrasound::Ultrasound;
use traits::*;
use gpio_cdev::{Chip};

fn main() {
    println!("Hello, world!");
    let mut rc = CarController::new();
    println!("\n{:?}", rc);
    rc.set_signal();
    println!("\n{:?}", rc.get_signal());
}
