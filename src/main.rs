mod robot;
mod ultrasound;
mod traits;

use ultrasound::Ultrasound;
use traits::*;
use gpio_cdev::{Chip};

fn main() {
    println!("Hello, world!");
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    println!("\n{:?}", chip);
    let sensor = Ultrasound::new(('s',0,1), &mut chip, 24, 18).unwrap();
    println!("\n{:?}", sensor);
    println!("distance {:?}", sensor.sense());
}
