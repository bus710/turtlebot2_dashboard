#![allow(unused)]

use std::sync::{Arc, Mutex};
// use std::convert::From;
// use std::time::Duration;
// use std::{thread, vec};

use anyhow::{anyhow, Error, Result};
use crossbeam_channel::unbounded;
use derivative::*;
use once_cell::sync::OnceCell;
use serialport::{SerialPortInfo, SerialPortType};
// use crossbeam_channel::internal::SelectHandle;
// use itertools::Itertools;

use flutter_rust_bridge::{StreamSink, SyncReturn};
// use flutter_rust_bridge::rust2dart::TaskCallback;
// use flutter_rust_bridge::support::{into_leak_vec_ptr, DartCObject, IntoDart};
// use flutter_rust_bridge::{StreamSink, SyncReturn, ZeroCopyBuffer};

use crate::turtlebot2::{Turtlebot, TurtlebotRunner};

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Feedback {
    // #[derivative(Default(value = "0"))]
    pub epoch_time_stamp: String,
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

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct BasicSensor {
    pub valid: bool,
    pub time_stamp: u32,
    pub bumper: u32,
    pub wheel_drop: u32,
    pub cliff: u32,
    pub left_encoder: u32,
    pub right_encoder: u32,
    pub left_pwm: u32,
    pub right_pwm: u32,
    pub button: u32,
    pub charger: u32,
    pub battery: u32,
    pub overcurrent_flags: u32,
}

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct DockingIR {
    pub valid: bool,
    pub right_signal: u32,
    pub central_signal: u32,
    pub left_signal: u32,
}
#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct InertialSensor {
    pub valid: bool,
    pub angle: u32,
    pub angle_rate: u32,
}
#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Cliff {
    pub valid: bool,
    pub right_cliff_sensor: u32,
    pub central_cliff_sensor: u32,
    pub left_cliff_sensor: u32,
}
#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Current {
    pub valid: bool,
    pub left_motor: u32,
    pub right_motor: u32,
}
#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct HardwareVersion {
    pub valid: bool,
    pub patch: u32,
    pub minor: u32,
    pub major: u32,
}
#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct FirmwareVersion {
    pub valid: bool,
    pub patch: u32,
    pub minor: u32,
    pub major: u32,
}
#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Gyro {
    pub valid: bool,
    pub frame_id: u32,
    pub followed_data_length: u32,
    pub x0: u32,
    pub y0: u32,
    pub z0: u32,
    pub x1: u32,
    pub y1: u32,
    pub z1: u32,
    pub x2: u32,
    pub y2: u32,
    pub z2: u32,
}

#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct GeneralPurposeInput {
    pub valid: bool,
    pub d_ch0: u32,
    pub a_ch0: u32,
    pub a_ch1: u32,
    pub a_ch2: u32,
    pub a_ch3: u32,
}
#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct UniqueDeviceId {
    pub valid: bool,
    pub udid0: u32,
    pub udid1: u32,
    pub udid2: u32,
}
#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct ControllerInfo {
    pub valid: bool,
    pub is_user_configured: u32,
    pub p_gain: u32,
    pub i_gain: u32,
    pub d_gain: u32,
}

impl Feedback {
    pub fn new() -> Feedback {
        Feedback {
            epoch_time_stamp: "".to_string(),
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

//
#[derive(Debug, Clone)]
pub struct Command {
    pub ty: String,
    pub opt: String,
    pub payload: Vec<u8>,
}

// Keyword to find USB-Serial devices
const SERIAL: &str = "kobuki";
// Static channel to interact with turtlebot
static SEND: OnceCell<Arc<Mutex<crossbeam_channel::Sender<Command>>>> = OnceCell::new();
// Static vector to store feedbacks from turtlebot
static RECEIVE: OnceCell<Arc<Mutex<Vec<Feedback>>>> = OnceCell::new();

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
    // The global static SEND is used to send command to the turtlebot instance
    let (sender, receiver) = unbounded();
    let sender_lock = Arc::new(Mutex::new(sender));
    SEND.set(sender_lock);

    // The receiver is passed to the turtlebot instance so flutter can send command to turtlebot
    // The sink is passed to the turtlebot instance so it can actively send result to flutter
    // => then flutter should call receive_from_turtlebot to take the data
    let mut ttb_runner = TurtlebotRunner::new(receiver, sink);
    ttb_runner.run();

    Ok(())
}

pub fn send_to_turtlebot(command: Command) -> Result<()> {
    let tx_lock = SEND.get().unwrap();
    let tx = tx_lock.lock().unwrap();
    tx.send(command);
    Ok(())
}

pub fn open_port_command(port: String) -> Result<()> {
    let mut cmd = Command {
        ty: "serial".to_string(),
        opt: port,
        payload: Vec::new(),
    };
    let tx_lock = SEND.get().unwrap();
    let tx = tx_lock.lock().unwrap();
    tx.send(cmd);

    Ok(())
}

pub fn receive_from_turtlebot() -> Result<Vec<Feedback>> {
    let fbd_lock = RECEIVE.get().unwrap();
    let fbd = fbd_lock.lock().unwrap();
    if fbd.len() > 0 {
        return Ok(fbd.clone());
    }

    Err(anyhow!(""))
}
