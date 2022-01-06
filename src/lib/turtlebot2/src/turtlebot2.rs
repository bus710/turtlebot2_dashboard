#![allow(unused)]

use std::ops::Shl;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use thiserror::Error;

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

pub fn decode(len: usize, buffer: &[u8], mut residue: &[u8]) -> Result<(Vec<Feedback>, Vec<u8>)> {
    // Check if the length if enough.
    // The min length is 70.
    if len < 70 {
        eprintln!("Not enough - {:?}", len);
        return Err(anyhow!(""));
    }

    eprintln!("total - {:?}", len);
    eprintln!();

    let mut feedbacks = Vec::new();
    let mut new_residue = Vec::new();

    // search for the preambles (0xaa, 0x55)
    let headers = search_header(&buffer[..len]).expect("Headers not found");
    eprintln!("header indexes - {:?}", headers);
    eprintln!();

    // if the first preambles set is not located at 0 of the buffer,
    // the residue from previous iteration should be used to make a complete packet
    if headers[0] != 0 {
        // eprintln!("residue packet - {:?}", residue);
        // eprintln!("broken packet - {:?}", &buffer[..headers[0]]);
        let broken_packet = &buffer[..headers[0]];
        if residue.len() != 0 {
            let merged_packet = merge_residue(&residue, broken_packet).expect("");
            let correct_crc = check_crc(&merged_packet);
            eprintln!(
                "residue & broken (crc: {:?}) - {:?}",
                correct_crc, merged_packet
            );
            eprintln!();
            if correct_crc {
                let f = format_feedback(&merged_packet).expect("msg");
                feedbacks.push(f);
                // eprintln!("feedback - {:?}", f);
            }
        }
        eprintln!();
    }

    // divide packets by header found
    let raw_packets = divide_packet(&buffer[..len], &headers).expect("Packets not found");
    for (i, raw_packet) in raw_packets.iter().enumerate() {
        // check CRC and set the residue to pass to next iteration.
        let correct_crc = check_crc(&raw_packet.clone());
        eprintln!(
            "raw packet (index: {:?}, crc: {:?}) - {:?}",
            i,
            correct_crc,
            raw_packet.as_slice()
        );

        if !correct_crc {
            if i == 0 {
                eprintln!("CRC not matched - residue from previous seems not used");
            } else {
                eprintln!("CRC not matched - pass to the next");
            }
            new_residue = raw_packet.clone(); // pass to the next iteration
        } else {
            let f = format_feedback(raw_packet).expect("");
            // eprintln!("feedback - {:?}", f);
            feedbacks.push(f);
            new_residue = [0u8; 0].to_vec(); // clear so don't pass to the next iteration
        }
        eprintln!();
    }
    eprintln!();

    return Ok((feedbacks, new_residue));
}

pub fn merge_residue(residue: &[u8], broken_packet: &[u8]) -> Result<Vec<u8>> {
    let mut a = residue.clone().to_vec();
    let b = broken_packet.to_vec();
    a.extend(b);

    Ok(a)
}

pub fn search_header(packet: &[u8]) -> Result<Vec<usize>> {
    let mut h = Vec::new();
    let buf = packet.iter();
    // Need to skip the first byte since this loop accesses [index-1]
    for (i, c) in buf.enumerate().skip(1) {
        if packet[i - 1] == 0xaa && packet[i] == 0x55 {
            h.push(i - 1);
        }
    }
    Ok(h)
}

pub fn divide_packet(packet: &[u8], h: &[usize]) -> Result<Vec<Vec<u8>>> {
    let mut p = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for (i, c) in h.iter().enumerate() {
        // use the indixes from h.
        // or use bytes until the end if currently last part.
        if i + 1 != h.len() {
            start = h[i];
            end = h[i + 1];
        } else {
            start = h[i];
            end = packet.len(); // until the end => residue
        }
        // eprintln!("s/e - {:?}/{:?}", start, end);
        let b = packet[start..end].to_vec().clone();
        p.push(b);
    }
    Ok(p)
}

pub fn check_crc(packet: &Vec<u8>) -> bool {
    let last = packet.len() - 1;
    let checksum = packet.as_slice()[last];
    let mut acc: u8 = 0;

    for (i, c) in packet.iter().enumerate().skip(2) {
        if i == last {
            break;
        }
        acc = acc ^ *c;
    }
    acc == checksum
}

use std::time::{SystemTime, UNIX_EPOCH};

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn format_feedback(raw_packet: &Vec<u8>) -> Result<Feedback> {
    let total_len = raw_packet[2].clone();
    let mut exit_count = 0;
    let mut index: u8 = 3; // assign the index of first ID of a feedback
    let mut f = Feedback::new();

    f.epoch_time_stamp = get_epoch_ms();

    loop {
        exit_count += 1;
        if index >= total_len || exit_count > 20 {
            break;
        }
        let payload_index = raw_packet[index as usize];
        let id = num::FromPrimitive::from_u8(payload_index);

        match id {
            Some(FeedbackId::BasicSensor) => {
                f.basic_sensor.valid = true;
                f.basic_sensor.time_stamp = raw_packet[2 + index as usize] as u16;
                f.basic_sensor.time_stamp |= (raw_packet[3 + index as usize] as u16).shl(8);
                f.basic_sensor.bumper = raw_packet[4 + index as usize];
                f.basic_sensor.wheel_drop = raw_packet[5 + index as usize];
                f.basic_sensor.cliff = raw_packet[6 + index as usize];
                f.basic_sensor.left_encoder = raw_packet[7 + index as usize] as u16;
                f.basic_sensor.left_encoder |= (raw_packet[8 + index as usize] as u16).shl(8);
                f.basic_sensor.right_encoder = raw_packet[9 + index as usize] as u16;
                f.basic_sensor.right_encoder |= (raw_packet[10 + index as usize] as u16).shl(8);
                f.basic_sensor.left_pwm = raw_packet[11 + index as usize];
                f.basic_sensor.right_pwm = raw_packet[12 + index as usize];
                f.basic_sensor.button = raw_packet[13 + index as usize];
                f.basic_sensor.charger = raw_packet[14 + index as usize];
                f.basic_sensor.battery = raw_packet[15 + index as usize];
                f.basic_sensor.overcurrent_flags = raw_packet[16 + index as usize];
                index += FDB_SIZE_BASIC_SENSOR_DATA + 2;
            }
            Some(FeedbackId::DockingIR) => {
                f.docking_ir.valid = true;
                f.docking_ir.right_signal = raw_packet[2 + index as usize];
                f.docking_ir.central_signal = raw_packet[3 + index as usize];
                f.docking_ir.left_signal = raw_packet[4 + index as usize];
                index += FDB_SIZE_DOCKING_IR + 2;
            }
            Some(FeedbackId::InertialSensor) => {
                f.inertial_sensor.valid = true;
                f.inertial_sensor.angle = raw_packet[2 + index as usize] as u16;
                f.inertial_sensor.angle |= (raw_packet[3 + index as usize] as u16).shl(8);
                f.inertial_sensor.angle_rate = raw_packet[4 + index as usize] as u16;
                f.inertial_sensor.angle_rate |= (raw_packet[5 + index as usize] as u16).shl(8);
                index += FDB_SIZE_INERTIAL_SENSOR + 2;
            }
            Some(FeedbackId::Cliff) => {
                f.cliff.valid = true;
                f.cliff.right_cliff_sensor = raw_packet[2 + index as usize] as u16;
                f.cliff.right_cliff_sensor |= (raw_packet[3 + index as usize] as u16).shl(8);
                f.cliff.central_cliff_sensor = raw_packet[4 + index as usize] as u16;
                f.cliff.central_cliff_sensor |= (raw_packet[5 + index as usize] as u16).shl(8);
                f.cliff.left_cliff_sensor = raw_packet[6 + index as usize] as u16;
                f.cliff.left_cliff_sensor |= (raw_packet[7 + index as usize] as u16).shl(8);
                index += FDB_SIZE_CLIFF + 2;
            }
            Some(FeedbackId::Current) => {
                f.current.valid = true;
                f.current.left_motor = raw_packet[2 + index as usize];
                f.current.right_motor = raw_packet[3 + index as usize];
                index += FDB_SIZE_CURRENT + 2;
            }
            Some(FeedbackId::HardwareVersion) => {
                f.hardware_version.valid = true;
                f.hardware_version.patch = raw_packet[2 + index as usize];
                f.hardware_version.minor = raw_packet[3 + index as usize];
                f.hardware_version.major = raw_packet[4 + index as usize];
                index += FDB_SIZE_HARDWARE_VERSION + 2;
            }
            Some(FeedbackId::FirmwareVersion) => {
                f.firmware_version.valid = true;
                f.firmware_version.patch = raw_packet[2 + index as usize];
                f.firmware_version.minor = raw_packet[3 + index as usize];
                f.firmware_version.major = raw_packet[4 + index as usize];
                index += FDB_SIZE_FIRMWARE_VERSION + 2;
            }
            Some(FeedbackId::RawDataOf3AxisGyro) => {
                f.gyro.valid = true;
                f.gyro.frame_id = raw_packet[2 + index as usize];
                f.gyro.followed_data_length = raw_packet[3 + index as usize];
                //
                f.gyro.raw_gyro_data[0].x = raw_packet[4 + index as usize] as u16;
                f.gyro.raw_gyro_data[0].x |= (raw_packet[5 + index as usize] as u16).shl(8);
                f.gyro.raw_gyro_data[0].y = raw_packet[6 + index as usize] as u16;
                f.gyro.raw_gyro_data[0].y |= (raw_packet[7 + index as usize] as u16).shl(8);
                f.gyro.raw_gyro_data[0].z = raw_packet[8 + index as usize] as u16;
                f.gyro.raw_gyro_data[0].z |= (raw_packet[9 + index as usize] as u16).shl(8);
                //
                f.gyro.raw_gyro_data[1].x = raw_packet[10 + index as usize] as u16;
                f.gyro.raw_gyro_data[1].x |= (raw_packet[11 + index as usize] as u16).shl(8);
                f.gyro.raw_gyro_data[1].y = raw_packet[12 + index as usize] as u16;
                f.gyro.raw_gyro_data[1].y |= (raw_packet[13 + index as usize] as u16).shl(8);
                f.gyro.raw_gyro_data[1].z = raw_packet[14 + index as usize] as u16;
                f.gyro.raw_gyro_data[1].z |= (raw_packet[15 + index as usize] as u16).shl(8);
                //
                if raw_packet[1 + index as usize] == FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A {
                    index += FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A + 2;
                } else if raw_packet[1 + index as usize] == FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B {
                    f.gyro.raw_gyro_data[2].x = raw_packet[16 + index as usize] as u16;
                    f.gyro.raw_gyro_data[2].x |= (raw_packet[17 + index as usize] as u16).shl(8);
                    f.gyro.raw_gyro_data[2].y = raw_packet[18 + index as usize] as u16;
                    f.gyro.raw_gyro_data[2].y |= (raw_packet[19 + index as usize] as u16).shl(8);
                    f.gyro.raw_gyro_data[2].z = raw_packet[20 + index as usize] as u16;
                    f.gyro.raw_gyro_data[2].z |= (raw_packet[21 + index as usize] as u16).shl(8);
                    index += FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B + 2;
                }
            }
            Some(FeedbackId::GeneralPurposeInput) => {
                f.general_purpose_input.valid = true;
                f.general_purpose_input.d_ch0 = raw_packet[2 + index as usize] as u16;
                f.general_purpose_input.d_ch0 |= (raw_packet[3 + index as usize] as u16).shl(8);
                //
                f.general_purpose_input.a_ch0 = raw_packet[4 + index as usize] as u16;
                f.general_purpose_input.a_ch0 |= (raw_packet[5 + index as usize] as u16).shl(8);
                //
                f.general_purpose_input.a_ch1 = raw_packet[6 + index as usize] as u16;
                f.general_purpose_input.a_ch1 |= (raw_packet[7 + index as usize] as u16).shl(8);
                //
                f.general_purpose_input.a_ch2 = raw_packet[8 + index as usize] as u16;
                f.general_purpose_input.a_ch2 |= (raw_packet[9 + index as usize] as u16).shl(8);
                //
                f.general_purpose_input.a_ch3 = raw_packet[10 + index as usize] as u16;
                f.general_purpose_input.a_ch3 |= (raw_packet[11 + index as usize] as u16).shl(8);
                index += FDB_SIZE_GENERAL_PURPOSE_OUTPUT + 2;
            }
            Some(FeedbackId::UniqueDeviceId) => {
                f.unique_device_id.valid = true;
                f.unique_device_id.udid0 = raw_packet[2 + index as usize] as u32;
                f.unique_device_id.udid0 |= (raw_packet[3 + index as usize] as u32).shl(8);
                f.unique_device_id.udid0 |= (raw_packet[4 + index as usize] as u32).shl(16);
                f.unique_device_id.udid0 |= (raw_packet[5 + index as usize] as u32).shl(24);
                //
                f.unique_device_id.udid1 = raw_packet[6 + index as usize] as u32;
                f.unique_device_id.udid1 |= (raw_packet[7 + index as usize] as u32).shl(8);
                f.unique_device_id.udid1 |= (raw_packet[8 + index as usize] as u32).shl(16);
                f.unique_device_id.udid1 |= (raw_packet[9 + index as usize] as u32).shl(24);
                //
                f.unique_device_id.udid2 = raw_packet[10 + index as usize] as u32;
                f.unique_device_id.udid2 |= (raw_packet[11 + index as usize] as u32).shl(8);
                f.unique_device_id.udid2 |= (raw_packet[12 + index as usize] as u32).shl(16);
                f.unique_device_id.udid2 |= (raw_packet[13 + index as usize] as u32).shl(24);
                index += FDB_SIZE_UNIQUE_DEVICE_IDENTIFIER + 2;
            }
            Some(FeedbackId::ControllerInfo) => {
                f.controller_info.valid = true;
                f.controller_info.p_gain = raw_packet[2 + index as usize] as u32;
                f.controller_info.p_gain |= (raw_packet[3 + index as usize] as u32).shl(8);
                f.controller_info.p_gain |= (raw_packet[4 + index as usize] as u32).shl(16);
                f.controller_info.p_gain |= (raw_packet[5 + index as usize] as u32).shl(24);
                //
                f.controller_info.i_gain = raw_packet[6 + index as usize] as u32;
                f.controller_info.i_gain |= (raw_packet[7 + index as usize] as u32).shl(8);
                f.controller_info.i_gain |= (raw_packet[8 + index as usize] as u32).shl(16);
                f.controller_info.i_gain |= (raw_packet[9 + index as usize] as u32).shl(24);
                //
                f.controller_info.d_gain = raw_packet[10 + index as usize] as u32;
                f.controller_info.d_gain |= (raw_packet[11 + index as usize] as u32).shl(8);
                f.controller_info.d_gain |= (raw_packet[12 + index as usize] as u32).shl(16);
                f.controller_info.d_gain |= (raw_packet[13 + index as usize] as u32).shl(24);
                index += FDB_SIZE_CONTROLLER_INFO + 2;
            }
            _ => {
                //
            }
        }
    }
    Ok(f)
}
