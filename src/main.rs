extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::sensors::UltrasonicSensor;

fn main() -> Ev3Result<()> {

    let Ultrasonicsensor = UltrasonicSensor::find()?;

     
    println!("Current distance: {}", Ultrasonicsensor.get_distance_centimeters().unwrap());

    Ok(())
}
