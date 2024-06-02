use core::ops::Range;

use ufmt::derive::uDebug;

pub struct Registers;
impl Registers {
    pub const CHIPID: Range<u8> = 0x00..0x00;
    pub const ERR_REG: Range<u8> = 0x02..0x02;
    pub const PMU_STATUS: Range<u8> = 0x03..0x03;
    pub const STATUS: Range<u8> = 0x1B..0x1B;
    pub const MAG: Range<u8> = 0x04..0x0B;
    pub const GYR: Range<u8> = 0x0C..0x11;
    pub const ACC: Range<u8> = 0x12..0x17;
    pub const SENSORTIME: Range<u8> = 0x18..0x1A;
    pub const INT_EN: Range<u8> = 0x50..0x52;
    pub const CMD: u8 = 0x7E;
}

#[derive(uDebug)]
pub enum AccelerometerPowerMode { Suspend, LowPower, Normal }
#[derive(uDebug)]
pub enum MagnetometerPowerMode { Suspend, LowPower, Normal }
#[derive(uDebug)]
pub enum GyroscopePowerMode { Suspend, Reserved, Normal, FastStartUp }

#[derive(uDebug)]
pub struct PowerMode {
    accelerometer: AccelerometerPowerMode,
    magnetometer: MagnetometerPowerMode,
    gyrosope: GyroscopePowerMode,
}

struct StatusFlags;
impl StatusFlags {
    pub const DRDY_ACC: u8 = 1 << 7;
    pub const DRDY_GYR: u8 = 1 << 6;
    pub const DRDY_MAG: u8 = 1 << 5;
    pub const NVM_RDY: u8 = 1 << 4;
    pub const FOC_RDY: u8 = 1 << 3;
    pub const MAG_MAN_OP: u8 = 1 << 2;
    pub const GYR_SELF_TEST_OK: u8 = 1 << 1;
}

#[derive(uDebug)]
pub struct Status {
    // Data ready (DRDY) for accelerometer in register
    pub drdy_acc: bool,
    // Data ready (DRDY) for gyroscope in register
    pub drdy_gyr: bool,
    // Data ready (DRDY) for magnetometer in registe
    pub drdy_mag: bool,
    // NVM controller status
    pub nvm_rdy: bool,
    // FOC completed
    pub foc_rdy: bool,
    // Manual magnetometer interface operation
    pub mag_man_op: bool,
    // Gyroscope self-test completed successfully
    pub gyr_self_test_ok: bool,
}

#[derive(uDebug)]
pub struct SensorData {
    x: i16,
    y: i16,
    z: i16,
}

// The status of the interupt engines
#[derive(uDebug)]
pub struct Interupts {
    pub any_motion_x: bool,
    pub any_motion_y: bool,
    pub any_motion_z: bool,
    pub double_tap: bool,
    pub single_tap: bool,
    pub orientation: bool,
    pub flat: bool,
    pub high_g_x: bool,
    pub high_g_y: bool,
    pub high_g_z: bool,
    pub low_g: bool,
    pub drdy: bool,
    pub fifo_full: bool,
    pub fifo_watermark: bool,
    pub no_or_slow_motion_x: bool,
    pub no_or_slow_motion_y: bool,
    pub no_or_slow_motion_z: bool,
    pub step_detector: bool,
}

pub fn parse_interupts(data: &[u8]) -> Interupts {
    Interupts {
        any_motion_x: data[0] & 0b0000_0001 != 0,
        any_motion_y: data[0] & 0b0000_0010 != 0,
        any_motion_z: data[0] & 0b0000_0100 != 0,
        double_tap: data[0] & 0b0001_0000 != 0,
        single_tap: data[0] & 0b0010_0000 != 0,
        orientation: data[0] & 0b0100_0000 != 0,
        flat: data[0] & 0b1000_0000 != 0,
        high_g_x: data[1] & 0b0000_0001 != 0,
        high_g_y: data[1] & 0b0000_0010 != 0,
        high_g_z: data[1] & 0b0000_0100 != 0,
        low_g: data[1] & 0b0000_1000 != 0,
        drdy: data[1] & 0b0001_0000 != 0,
        fifo_full: data[1] & 0b0010_0000 != 0,
        fifo_watermark: data[1] & 0b0100_0000 != 0,
        no_or_slow_motion_x: data[2] & 0b0000_0001 != 0,
        no_or_slow_motion_y: data[2] & 0b0000_0010 != 0,
        no_or_slow_motion_z: data[2] & 0b0000_0100 != 0,
        step_detector: data[2] & 0b0000_1000 != 0,
    }
}

pub fn parse_status(status: u8) -> Status {
    Status {
        drdy_acc: status & StatusFlags::DRDY_ACC != 0,
        drdy_gyr: status & StatusFlags::DRDY_GYR != 0,
        drdy_mag: status & StatusFlags::DRDY_MAG != 0,
        nvm_rdy: status & StatusFlags::NVM_RDY != 0,
        foc_rdy: status & StatusFlags::FOC_RDY != 0,
        mag_man_op: status & StatusFlags::MAG_MAN_OP != 0,
        gyr_self_test_ok: status & StatusFlags::GYR_SELF_TEST_OK != 0,
    }
}

pub fn parse_power_mode(power_mode: u8) -> PowerMode {
    PowerMode {
        accelerometer: match power_mode >> 4 & 0b11 {
            0b00 => AccelerometerPowerMode::Suspend,
            0b01 => AccelerometerPowerMode::Normal,
            0b10 => AccelerometerPowerMode::LowPower,
            _ => AccelerometerPowerMode::Normal,
        },
        magnetometer: match power_mode & 0b11 {
            0b00 => MagnetometerPowerMode::Suspend,
            0b01 => MagnetometerPowerMode::Normal,
            0b10 => MagnetometerPowerMode::LowPower,
            _ => MagnetometerPowerMode::Normal,
        },
        gyrosope: match power_mode >> 2 & 0b11 {
            0b00 => GyroscopePowerMode::Suspend,
            0b01 => GyroscopePowerMode::Normal,
            0b10 => GyroscopePowerMode::Reserved,
            0b11 => GyroscopePowerMode::FastStartUp,
            _ => GyroscopePowerMode::Normal,
        },
    }
}

pub fn parse_sensor_data(data: &[u8]) -> SensorData {
    SensorData {
        x: (u16::from(data[0]) | u16::from(data[1]) << 8) as i16,
        y: (u16::from(data[2]) | u16::from(data[3]) << 8) as i16,
        z: (u16::from(data[4]) | u16::from(data[5]) << 8) as i16,
    }
}

