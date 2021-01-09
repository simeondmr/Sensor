use crate::sensors::sensor::SensorType::PHOTORESISTOR;

pub struct Sens {
    pub pin: i8,
    s_type: SensorType
}

impl Sens{
    pub fn new(pin: i8, s_type: SensorType) -> Sens {
        Sens {
            pin,
            s_type
        }
    }

    pub fn get_pin(&self) -> i8 {
        self.pin
    }
}

pub enum SensorType {
    PHOTORESISTOR,
    THERMISTOR
}

pub trait SensorFuncs {
    fn read(&mut self) -> Option<i16>;
}