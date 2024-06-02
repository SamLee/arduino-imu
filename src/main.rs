#![no_std]
#![no_main]

use core::ops::Range;

use arduino_hal::I2c;
use bmi160::parse_interupts;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use panic_halt as _;

const I2C_ADDR: u8 = 0x68;

mod bmi160;

fn read_reg(i2c: &mut I2c, register: Range<u8>, buffer: &mut [u8]) {
    let len = register.len() + 1;
    i2c.write_read(I2C_ADDR, &[register.start], &mut buffer[0..len]).unwrap();
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000
    );

    let mut result = [0; 8];
    ufmt::uwriteln!(&mut serial, "Searching for i2c device").unwrap();
    read_reg(&mut i2c, bmi160::Registers::CHIPID, &mut result);
    ufmt::uwriteln!(&mut serial, "Chip id: {:?}", result).unwrap();
    read_reg(&mut i2c, bmi160::Registers::ERR_REG, &mut result);
    ufmt::uwriteln!(&mut serial, "Error register: {:?}", result).unwrap();
    read_reg(&mut i2c, bmi160::Registers::STATUS, &mut result);
    ufmt::uwriteln!(&mut serial, "Status register: {:?}", bmi160::parse_status(result[0])).unwrap();
    read_reg(&mut i2c, bmi160::Registers::PMU_STATUS, &mut result);
    ufmt::uwriteln!(&mut serial, "Power mode: {:?}", bmi160::parse_power_mode(result[0])).unwrap();
    read_reg(&mut i2c, bmi160::Registers::SENSORTIME, &mut result);
    ufmt::uwriteln!(
        &mut serial,
        "Sensor Time: {:?} -> {:?}",
        result[0..3],
        u32::from(result[0]) | (u32::from(result[1]) << 8) | (u32::from(result[2]) << 16)
    ).unwrap();
    read_reg(&mut i2c, bmi160::Registers::INT_EN, &mut result);
    ufmt::uwriteln!(&mut serial, "INT_EN: {:?}", parse_interupts(&result[0..3])).unwrap();


    // Enable accelerometer
    i2c.write(I2C_ADDR, &[bmi160::Registers::CMD, 0b0001_0001]).unwrap();
    // Enable gyroscope
    i2c.write(I2C_ADDR, &[bmi160::Registers::CMD, 0b0001_0101]).unwrap();

    loop {
        read_reg(&mut i2c, bmi160::Registers::STATUS, &mut result);
        let status = bmi160::parse_status(result[0]);
        if status.drdy_acc {
            read_reg(&mut i2c, bmi160::Registers::ACC, &mut result);
            ufmt::uwriteln!(&mut serial, "Acc: {:?}", bmi160::parse_sensor_data(&result)).unwrap();
        } else {
            ufmt::uwriteln!(&mut serial, "Acc: Not ready").unwrap();
        }

        if status.drdy_gyr {
            read_reg(&mut i2c, bmi160::Registers::GYR, &mut result);
            ufmt::uwriteln!(&mut serial, "Gyr: {:?}", bmi160::parse_sensor_data(&result)).unwrap();
        } else {
            ufmt::uwriteln!(&mut serial, "Gyr: Not ready").unwrap();
        }

        arduino_hal::delay_ms(1000);
    }
}
