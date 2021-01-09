use crate::sensors::sensor::{Sens, SensorFuncs};
use linux_embedded_hal::I2cdev;
use nb::block;
use ads1x1x::{channel, Ads1x1x, SlaveAddr};
use ads1x1x::mode::OneShot;
use ads1x1x::interface::I2cInterface;
use ads1x1x::ic::{Ads1115, Resolution16Bit};
use std::cell::Ref;
use embedded_hal::adc::OneShot as EmbeddedHalOneShot;

pub struct AnalogSensor {
    sensor: Sens,
    adc:  Ads1x1x<I2cInterface<I2cdev>, Ads1115, Resolution16Bit, OneShot>
}

impl AnalogSensor {
    pub fn new(sensor: Sens) -> AnalogSensor {
        let dev = I2cdev::new("/dev/i2c-1").unwrap();
        let address = SlaveAddr::default();
        let adc = Ads1x1x::new_ads1115(dev, address);
        AnalogSensor {
            sensor,
            adc
        }
    }
}

impl SensorFuncs for AnalogSensor {
    fn read(&mut self) -> Option<i16> {
        match self.sensor.pin {
            0 => Some(block!(self.adc.read(&mut channel::DifferentialA0A1)).unwrap()),
            1 => Some(block!(self.adc.read(&mut channel::DifferentialA0A1)).unwrap()),
            2 => Some(block!(self.adc.read(&mut channel::DifferentialA0A1)).unwrap()),
            3 => Some(block!(self.adc.read(&mut channel::DifferentialA0A1)).unwrap()),
            _ => { None }
        }
    }
}