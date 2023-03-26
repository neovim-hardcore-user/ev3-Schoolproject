extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::{TachoMotor, MotorPort};
use ev3dev_lang_rust::sensors::UltrasonicSensor;

fn main() -> Ev3Result<()> {
    let ultrasonicsensor = UltrasonicSensor::find()?;

    let radar_motor = TachoMotor::get(MotorPort::OutD)?;
    
    radar_motor.reset();

    radar_motor.run_direct()?;

    radar_motor.set_duty_cycle_sp(50)?;
    while true {
        let position: f32 = radar_motor.get_position()? as f32;
        
        if position >= 70f32 {
            radar_motor.set_duty_cycle_sp(-50)?;
        } else if position <= -70f32 {
            radar_motor.set_duty_cycle_sp(50)?;
        }

        println!("distance: {}, motor position: {}", ultrasonicsensor.get_distance_centimeters().unwrap(), position);
    }
    Ok(())
}
