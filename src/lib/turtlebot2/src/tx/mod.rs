#![allow(unused)]

// Variant enum
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum FeedbackId {
    BasicSensor = 1,
    DockingIR = 3,
    InertialSensor = 4,
    Cliff = 5,
    Current = 6,
    HardwareVersion = 10,
    FirmwareVersion = 11,
    RawDataOf3AxisGyro = 13,
    GeneralPurposeInput = 16,
    UniqueDeviceId = 19,
    ControllerInfo = 21,
}

// These can be used to get the size of payload
// Gyro sensor size can be 14 or 20 bytes
pub const FDB_SIZE_BASIC_SENSOR_DATA: u8 = 15;
pub const FDB_SIZE_DOCKING_IR: u8 = 3;
pub const FDB_SIZE_INERTIAL_SENSOR: u8 = 7;
pub const FDB_SIZE_CLIFF: u8 = 6;
pub const FDB_SIZE_CURRENT: u8 = 2;
pub const FDB_SIZE_HARDWARE_VERSION: u8 = 4;
pub const FDB_SIZE_FIRMWARE_VERSION: u8 = 4;
pub const FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A: u8 = 14;
pub const FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B: u8 = 20;
pub const FDB_SIZE_GENERAL_PURPOSE_OUTPUT: u8 = 16;
pub const FDB_SIZE_UNIQUE_DEVICE_IDENTIFIER: u8 = 12;
pub const FDB_SIZE_CONTROLLER_INFO: u8 = 13;
