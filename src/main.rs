extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::UltrasonicSensor;
use ev3dev_lang_rust::sound;

const FOV: f32 = 70f32;

const speed: i32 = 30i32;


fn main() -> Ev3Result<()> {
    let ultrasonicsensor = UltrasonicSensor::find()?;

    let radar_motor = LargeMotor::get(MotorPort::OutD)?;


    sound::speak("Hello")?.wait()?;
    
    radar_motor.reset();

    radar_motor.run_direct()?;

    radar_motor.set_duty_cycle_sp(speed)?;

    
    while true {
        let position: f32 = radar_motor.get_position()? as f32;
        
        if position >= FOV {
            radar_motor.set_duty_cycle_sp(-speed)?;
        } else if position <= -FOV {
            radar_motor.set_duty_cycle_sp(speed)?;
        }

        println!("distance: {}, motor position: {}", ultrasonicsensor.get_distance_centimeters().unwrap(), position);
    }
    Ok(())
}
