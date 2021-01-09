use crate::sensors::sensor::{Sens, SensorFuncs};

impl SensorFuncs for Sens {
    fn read(&mut self) -> Option<i16> {
        unimplemented!()
    }
}