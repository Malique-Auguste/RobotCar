mod car;
mod car_control;
mod direction;
mod speed;
mod ultrasound;
mod traits;



use car_control::CarController;
use ultrasound::Ultrasound;
use traits::*;
use gpio_cdev::{Chip};

fn main() {
    println!("Hello, world!");
    let mut rc = CarController::new('c', 1, 1);
    println!("\n{:?}", rc);
    rc.set_signal();
    println!("\n{:?}", rc.get_signal());
}
