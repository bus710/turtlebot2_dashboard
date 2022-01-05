#![allow(unused)]

use derivative::*;

// ====================================

pub enum CommandId {
    BaseControl = 1,
    Sound = 3,
    SoundSequence = 4,
    RequestExtra = 9,
    GeneralPurposeOutput = 12,
    SetControllerGain = 13,
    GetControllerGain = 14,
}

// These can be used to set the length of command
// Total length = ID + Size + Payload + CRC
pub const CMD_LEN_BASE_CONTROL: u8 = 7;
pub const CMD_LEN_SOUND: u8 = 6;
pub const CMD_LEN_SOUND_SEQUENCE: u8 = 4;
pub const CMD_LEN_REQUEST_EXTRA: u8 = 5;
pub const CMD_LEN_GENERAL_PURPOSE_OUTPUT: u8 = 5;
pub const CMD_LEN_SET_CONTROLLER_GAIN: u8 = 16;
pub const CMD_LEN_GET_CONTROLLER_GAIN: u8 = 4;

// These can be used to set the size of payload
pub const CMD_SIZE_BASE_CONTROL: u8 = 4;
pub const CMD_SIZE_SOUND: u8 = 3;
pub const CMD_SIZE_SOUND_SEQUENCE: u8 = 1;
pub const CMD_SIZE_REQUEST_EXTRA: u8 = 2;
pub const CMD_SIZE_GENERAL_PURPOSE_OUTPUT: u8 = 2;
pub const CMD_SIZE_SET_CONTROLLER_GAIN: u8 = 13;
pub const CMD_SIZE_GET_CONTROLLER_GAIN: u8 = 1;

// ====================================

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

// ====================================

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Feedback {
    #[derivative(Default(value = "0"))]
    pub epoch_time_stamp: u128,
    pub basic_sensor: BasicSensor,
    pub docking_ir: DockingIR,
    pub inertial_sensor: InertialSensor,
    pub cliff: Cliff,
    pub current: Current,
    pub hardware_version: HardwareVersion,
    pub firmware_version: FirmwareVersion,
    pub gyro: Gyro,
    pub general_purpose_input: GeneralPurposeInput,
    pub unique_device_id: UniqueDeviceId,
    pub controller_info: ControllerInfo,
}

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct BasicSensor {
    pub valid: bool,
    pub time_stamp: u16,
    pub bumper: u8,
    pub wheel_drop: u8,
    pub cliff: u8,
    pub left_encoder: u16,
    pub right_encoder: u16,
    pub left_pwm: u8,
    pub right_pwm: u8,
    pub button: u8,
    pub charger: u8,
    pub battery: u8,
    pub overcurrent_flags: u8,
}

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct DockingIR {
    pub valid: bool,
    pub right_signal: u8,
    pub central_signal: u8,
    pub left_signal: u8,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct InertialSensor {
    pub valid: bool,
    pub angle: u16,
    pub angle_rate: u16,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Cliff {
    pub valid: bool,
    pub right_cliff_sensor: u16,
    pub central_cliff_sensor: u16,
    pub left_cliff_sensor: u16,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Current {
    pub valid: bool,
    pub left_motor: u8,
    pub right_motor: u8,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct HardwareVersion {
    pub valid: bool,
    pub patch: u8,
    pub minor: u8,
    pub major: u8,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct FirmwareVersion {
    pub valid: bool,
    pub patch: u8,
    pub minor: u8,
    pub major: u8,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Gyro {
    pub valid: bool,
    pub frame_id: u8,
    pub followed_data_length: u8,
    pub raw_gyro_data: [RawGyro; 3],
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct RawGyro {
    pub valid: bool,
    pub x: u16,
    pub y: u16,
    pub z: u16,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct GeneralPurposeInput {
    pub valid: bool,
    pub d_ch0: u16,
    pub a_ch0: u16,
    pub a_ch1: u16,
    pub a_ch2: u16,
    pub a_ch3: u16,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct UniqueDeviceId {
    pub valid: bool,
    pub udid0: u32,
    pub udid1: u32,
    pub udid2: u32,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct ControllerInfo {
    pub valid: bool,
    pub is_user_configured: u8,
    pub p_gain: u32,
    pub i_gain: u32,
    pub d_gain: u32,
}

impl Feedback {
    pub fn new() -> Feedback {
        Feedback {
            epoch_time_stamp: 0,
            basic_sensor: BasicSensor::default(),
            docking_ir: DockingIR::default(),
            inertial_sensor: InertialSensor::default(),
            cliff: Cliff::default(),
            current: Current::default(),
            hardware_version: HardwareVersion::default(),
            firmware_version: FirmwareVersion::default(),
            gyro: Gyro::default(),
            general_purpose_input: GeneralPurposeInput::default(),
            unique_device_id: UniqueDeviceId::default(),
            controller_info: ControllerInfo::default(),
        }
    }
}

pub fn hello() {
    eprintln!("{:?}", "parse");
}
