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

#[derive(FromPrimitive, ToPrimitive)]
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
    UniqueDeviceIdentifier = 19,
    ControllerInfo = 21,
}

// These can be used to get the size of payload
// Gyro sensor size can be 14 or 20 bytes
pub const FDB_SIZE_BASIC_SENSOR_DATA: u8 = 15;
pub const FDB_SIZE_DOCKING_IR: u8 = 3;
pub const FDB_SIZE_ITERTIAL_SENSOR: u8 = 7;
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
    pub available_content: i32,
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
    pub unique_device_identifier: UniqueDeviceIdentifier,
    pub controller_info: ControllerInfo,
}

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct BasicSensor {
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
    pub right_signal: u8,
    pub central_signal: u8,
    pub left_signal: u8,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct InertialSensor {
    pub angle: u16,
    pub andle_rate: u16,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Cliff {
    pub right_cliff_sensor: u16,
    pub central_cliff_sensor: u16,
    pub left_cliff_sensor: u16,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Current {
    pub left_motor: u8,
    pub right_motor: u8,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct HardwareVersion {
    pub patch: u8,
    pub minor: u8,
    pub major: u8,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct FirmwareVersion {
    pub patch: u8,
    pub minor: u8,
    pub major: u8,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Gyro {
    pub frame_id: u8,
    pub followed_data_length: u8,
    pub raw_gyro_data_array: [RawGyro; 3],
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct RawGyro {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct GeneralPurposeInput {
    pub digital_input: u16,
    pub analog_input_ch0: u16,
    pub analog_input_ch1: u16,
    pub analog_input_ch2: u16,
    pub analog_input_ch3: u16,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct UniqueDeviceIdentifier {
    pub udid0: u32,
    pub udid1: u32,
    pub udid2: u32,
}
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct ControllerInfo {
    pub is_user_configured: u8,
    pub p_gain: u32,
    pub i_gain: u32,
    pub d_gain: u32,
}

impl Feedback {
    pub fn new() -> Feedback {
        Feedback {
            available_content: 0,
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
            unique_device_identifier: UniqueDeviceIdentifier::default(),
            controller_info: ControllerInfo::default(),
        }
    }
}

pub fn hello() {
    eprintln!("{:?}", "parse");
}
