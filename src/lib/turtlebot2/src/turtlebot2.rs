#![allow(unused)]

// ====================================

enum CommandId {
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
const CMD_LEN_BASE_CONTROL: i32 = 7;
const CMD_LEN_SOUND: i32 = 6;
const CMD_LEN_SOUND_SEQUENCE: i32 = 4;
const CMD_LEN_REQUEST_EXTRA: i32 = 5;
const CMD_LEN_GENERAL_PURPOSE_OUTPUT: i32 = 5;
const CMD_LEN_SET_CONTROLLER_GAIN: i32 = 16;
const CMD_LEN_GET_CONTROLLER_GAIN: i32 = 4;

// These can be used to set the size of payload
const CMD_SIZE_BASE_CONTROL: i32 = 4;
const CMD_SIZE_SOUND: i32 = 3;
const CMD_SIZE_SOUND_SEQUENCE: i32 = 1;
const CMD_SIZE_REQUEST_EXTRA: i32 = 2;
const CMD_SIZE_GENERAL_PURPOSE_OUTPUT: i32 = 2;
const CMD_SIZE_SET_CONTROLLER_GAIN: i32 = 13;
const CMD_SIZE_GET_CONTROLLER_GAIN: i32 = 1;

// ====================================

enum FeedbackId {
    TimeStamp = 0,
    BasicSensorData = 1,
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
const FDB_SIZE_BASIC_SENSOR_DATA: i32 = 15;
const FDB_SIZE_DOCKING_IR: i32 = 3;
const FDB_SIZE_ITERTIAL_SENSOR: i32 = 7;
const FDB_SIZE_CLIFF: i32 = 6;
const FDB_SIZE_CURRENT: i32 = 2;
const FDB_SIZE_HARDWARE_VERSION: i32 = 4;
const FDB_SIZE_FIRMWARE_VERSION: i32 = 4;
const FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A: i32 = 14;
const FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B: i32 = 20;
const FDB_SIZE_GENERAL_PURPOSE_OUTPUT: i32 = 16;
const FDB_SIZE_UNIQUE_DEVICE_IDENTIFIER: i32 = 12;
const FDB_SIZE_CONTROLLER_INFO: i32 = 13;

// ====================================

struct Feedback {
    available_content: i32,
    time_stamp: u128,
    basic_sensor: BasicSensor,
    docking_ir: DockingIR,
    inertial_sensor: InertialSensor,
    cliff: Cliff,
    current: Current,
    hardware_version: HardwareVersion,
    firmware_version: FirmwareVersion,
    gyro: Gyro,
    general_purpose_input: GeneralPurposeInput,
    unique_device_identifier: UniqueDeviceIdentifier,
    controller_info: ControllerInfo,
}

struct BasicSensor {
    time_stamp: u16,
    bumper: u8,
    wheel_drop: u8,
    cliff: u8,
    left_encoder: u16,
    right_encoder: u16,
    left_pwm: u8,
    right_pwm: u8,
    button: u8,
    charger: u8,
    battery: u8,
    overcurrent_flags: u8,
}
struct DockingIR {
    right_signal: u8,
    central_signal: u8,
    left_signal: u8,
}
struct InertialSensor {
    angle: u16,
    andle_rate: u16,
}
struct Cliff {
    right_cliff_sensor: u16,
    central_cliff_sensor: u16,
    left_cliff_sensor: u16,
}
struct Current {
    left_motor: u8,
    right_motor: u8,
}
struct HardwareVersion {
    patch: u8,
    minor: u8,
    major: u8,
}
struct FirmwareVersion {
    patch: u8,
    minor: u8,
    major: u8,
}
struct Gyro {
    frame_id: u8,
    followed_data_length: u8,
    raw_gyro_data_array: [RawGyro; 3],
}

struct RawGyro {
    x: u16,
    y: u16,
    z: u16,
}

struct GeneralPurposeInput {
    digiral_input: u16,
    analog_input_ch0: u16,
    analog_input_ch1: u16,
    analog_input_ch2: u16,
    analog_input_ch3: u16,
}
struct UniqueDeviceIdentifier {
    udid0: u32,
    udid1: u32,
    udid2: u32,
}
struct ControllerInfo {
    is_user_configured: u8,
    p_gain: u32,
    i_gain: u32,
    d_gain: u32,
}

pub fn hello() {
    eprintln!("{:?}", "parse");
}
