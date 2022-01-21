#![allow(unused)]

use std::ops::{Shl, Shr};
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Error, Result};
use crossbeam::unbounded;
use crossbeam_channel as crossbeam;
use derivative::*;
use flutter_rust_bridge::{StreamSink, SyncReturn};
use once_cell::sync::OnceCell;
use serialport::{SerialPortInfo, SerialPortType};

use crate::turtlebot2::*;
use crate::tx::*;
use crate::rx::*;

// // Variant enum
// #[derive(Debug, FromPrimitive, ToPrimitive)]
// pub enum FeedbackId {
//     BasicSensor = 1,
//     DockingIR = 3,
//     InertialSensor = 4,
//     Cliff = 5,
//     Current = 6,
//     HardwareVersion = 10,
//     FirmwareVersion = 11,
//     RawDataOf3AxisGyro = 13,
//     GeneralPurposeInput = 16,
//     UniqueDeviceId = 19,
//     ControllerInfo = 21,
// }

// // These can be used to get the size of payload
// // Gyro sensor size can be 14 or 20 bytes
// pub const FDB_SIZE_BASIC_SENSOR_DATA: u8 = 15;
// pub const FDB_SIZE_DOCKING_IR: u8 = 3;
// pub const FDB_SIZE_INERTIAL_SENSOR: u8 = 7;
// pub const FDB_SIZE_CLIFF: u8 = 6;
// pub const FDB_SIZE_CURRENT: u8 = 2;
// pub const FDB_SIZE_HARDWARE_VERSION: u8 = 4;
// pub const FDB_SIZE_FIRMWARE_VERSION: u8 = 4;
// pub const FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A: u8 = 14;
// pub const FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B: u8 = 20;
// pub const FDB_SIZE_GENERAL_PURPOSE_OUTPUT: u8 = 16;
// pub const FDB_SIZE_UNIQUE_DEVICE_IDENTIFIER: u8 = 12;
// pub const FDB_SIZE_CONTROLLER_INFO: u8 = 13;

// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct Feedback {
//     pub epoch_time_stamp: String,
//     pub basic_sensor: BasicSensor,
//     pub docking_ir: DockingIR,
//     pub inertial_sensor: InertialSensor,
//     pub cliff: Cliff,
//     pub current: Current,
//     pub hardware_version: HardwareVersion,
//     pub firmware_version: FirmwareVersion,
//     pub gyro: Gyro,
//     pub general_purpose_input: GeneralPurposeInput,
//     pub unique_device_id: UniqueDeviceId,
//     pub controller_info: ControllerInfo,
// }

// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct BasicSensor {
//     pub valid: bool,
//     pub time_stamp: u32,
//     pub bumper: u32,
//     pub wheel_drop: u32,
//     pub cliff: u32,
//     pub left_encoder: u32,
//     pub right_encoder: u32,
//     pub left_pwm: u32,
//     pub right_pwm: u32,
//     pub button: u32,
//     pub charger: u32,
//     pub battery: u32,
//     pub overcurrent_flags: u32,
// }

// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct DockingIR {
//     pub valid: bool,
//     pub right_signal: u32,
//     pub central_signal: u32,
//     pub left_signal: u32,
// }
// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct InertialSensor {
//     pub valid: bool,
//     pub angle: u32,
//     pub angle_rate: u32,
// }
// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct Cliff {
//     pub valid: bool,
//     pub right_cliff_sensor: u32,
//     pub central_cliff_sensor: u32,
//     pub left_cliff_sensor: u32,
// }
// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct Current {
//     pub valid: bool,
//     pub left_motor: u32,
//     pub right_motor: u32,
// }
// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct HardwareVersion {
//     pub valid: bool,
//     pub patch: u32,
//     pub minor: u32,
//     pub major: u32,
// }
// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct FirmwareVersion {
//     pub valid: bool,
//     pub patch: u32,
//     pub minor: u32,
//     pub major: u32,
// }
// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct Gyro {
//     pub valid: bool,
//     pub frame_id: u32,
//     pub followed_data_length: u32,
//     pub x0: u32,
//     pub y0: u32,
//     pub z0: u32,
//     pub x1: u32,
//     pub y1: u32,
//     pub z1: u32,
//     pub x2: u32,
//     pub y2: u32,
//     pub z2: u32,
// }

// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct GeneralPurposeInput {
//     pub valid: bool,
//     pub d_ch0: u32,
//     pub a_ch0: u32,
//     pub a_ch1: u32,
//     pub a_ch2: u32,
//     pub a_ch3: u32,
// }
// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct UniqueDeviceId {
//     pub valid: bool,
//     pub udid0: u32,
//     pub udid1: u32,
//     pub udid2: u32,
// }
// #[derive(Debug, Clone, Derivative)]
// #[derivative(Default)]
// pub struct ControllerInfo {
//     pub valid: bool,
//     pub is_user_configured: u32,
//     pub p_gain: u32,
//     pub i_gain: u32,
//     pub d_gain: u32,
// }

// impl Feedback {
//     pub fn new() -> Feedback {
//         Feedback {
//             epoch_time_stamp: "".to_string(),
//             basic_sensor: BasicSensor::default(),
//             docking_ir: DockingIR::default(),
//             inertial_sensor: InertialSensor::default(),
//             cliff: Cliff::default(),
//             current: Current::default(),
//             hardware_version: HardwareVersion::default(),
//             firmware_version: FirmwareVersion::default(),
//             gyro: Gyro::default(),
//             general_purpose_input: GeneralPurposeInput::default(),
//             unique_device_id: UniqueDeviceId::default(),
//             controller_info: ControllerInfo::default(),
//         }
//     }
// }

// Keyword to find USB-Serial devices
const SERIAL: &str = "kobuki";

pub fn available_tutlebots() -> Result<Vec<String>> {
    let ports = serialport::available_ports()?;
    if ports.len() < 1 {
        return Err(anyhow!("No port found! (or check dialout group)"));
    }

    let mut found = Vec::new();

    // Need to check if there is any port that has serial number with the given string "kobuki"
    let mut found_kobuki = false;
    for p in ports.iter() {
        match p.port_type.clone() {
            SerialPortType::UsbPort(info) => {
                if info.serial_number.unwrap().contains(SERIAL) {
                    // eprintln!("Found port: {:?} - {:?}", p.port_name, p);
                    found_kobuki = true;
                    found.push(p.port_name.clone());
                }
            }
            _ => (),
        };
    }

    Ok(found)
}

pub fn spawn_turtlebot(sink: StreamSink<String>) -> Result<()> {
    let (sender, receiver) = crossbeam::unbounded();
    set_statics_in_turtlebot(sender);

    // The receiver is passed to the turtlebot instance so flutter can send command to turtlebot
    // The sink is passed to the turtlebot instance so it can actively send result to flutter
    // => then flutter should call receive_from_turtlebot to take the data
    let mut ttb = Turtlebot::new(receiver, sink);
    ttb.run();

    Ok(())
}

fn generate_crc(payload: &[u8]) -> u8 {
    if payload.len() < 5 {
        return 0;
    }
    let payload = payload[5..].to_vec();
    let mut acc = 0;
    for (_, c) in payload.iter().enumerate() {
        acc = acc ^ *c;
    }
    acc
}

// will be called by other command functions
fn send_to_turtlebot(cmd: Command) -> Result<()> {
    // let tx_lock = SEND.get().unwrap();
    // let tx = tx_lock.lock().unwrap();
    // tx.send(cmd);

    send(cmd);
    Ok(())
}

// can be called to receive feedbacks when Flutter side gets notification via stream
pub fn receive_from_turtlebot() -> Result<Vec<Feedback>> {
    let feedbacks = receive();
    match feedbacks {
        Ok(f) => Ok(f),
        Err(_) => Err(anyhow!("What feedback?")),
    }
}

pub fn open_port_command(serial_port: String) -> Result<()> {
    eprintln!("open_port_command");
    let mut cmd = Command {
        ty: CommandId::SerialControl,
        serial_command: "open".to_string(),
        serial_port_name: serial_port,
        payload: Vec::new(),
    };
    send_to_turtlebot(cmd);
    Ok(())
}

pub fn close_port_command() -> Result<()> {
    let mut cmd = Command {
        ty: CommandId::SerialControl,
        serial_command: "close".to_string(),
        serial_port_name: "".to_string(),
        payload: Vec::new(),
    };
    send_to_turtlebot(cmd);
    Ok(())
}

pub fn base_control_command(speed: u16, radius: u16) -> Result<()> {
    let mut payload: Vec<u8> = Vec::new();
    payload.push(0xaa);
    payload.push(0x55);
    payload.push(CMD_LEN_BASE_CONTROL);
    payload.push(CommandId::BaseControl as u8);
    payload.push(CMD_SIZE_BASE_CONTROL);
    payload.push((speed & 0xff) as u8);
    payload.push((speed & 0xff00).shr(8) as u8);
    payload.push((radius & 0xff) as u8);
    payload.push((radius & 0xff00).shr(8) as u8);
    let crc = generate_crc(&payload.clone());
    payload.push(crc);

    let mut cmd = Command {
        ty: CommandId::BaseControl,
        serial_command: "".to_string(),
        serial_port_name: "".to_string(),
        payload: payload,
    };
    send_to_turtlebot(cmd);
    Ok(())
}
