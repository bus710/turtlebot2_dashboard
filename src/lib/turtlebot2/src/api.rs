#![allow(unused)]

use std::ops::{Shl, Shr};

use anyhow::{anyhow, Error, Result};
use crossbeam::unbounded;
use crossbeam_channel as crossbeam;
use derivative::*;
use flutter_rust_bridge::{StreamSink, SyncReturn};

use crate::rx::*;
use crate::turtlebot2::*;
use crate::tx::*;

// All of these structs should be here so it can be recognized by FFIGen Bridge.
#[derive(Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Feedback {
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

// Should be called ONLY once
pub fn spawn_turtlebot(sink: StreamSink<String>) -> Result<()> {
    // The sender will be set as the static in turtlebot.rs
    let (sender, receiver) = crossbeam::unbounded();
    set_statics_in_turtlebot(sender);

    // The receiver is passed to the turtlebot instance so Flutter can send command to turtlebot
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
fn send_to_turtlebot(mut cmd: Command) -> Result<()> {
    if cmd.ty != CommandId::SerialControl {
        let crc = generate_crc(&cmd.payload.clone());
        cmd.payload.push(crc);
    }
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

pub fn search_port_command() -> Result<Vec<String>> {
    let ports = available_tutlebots();
    match ports {
        Ok(p) => {
            if p.len() > 0 {
                return Ok(p);
            } else {
                return Err(anyhow!("What port?"));
            }
        }
        Err(_) => Err(anyhow!("What port?")),
    }
}

pub fn open_port_command(serial_port: String) -> Result<()> {
    let mut cmd = Command::new();
    cmd.ty = CommandId::SerialControl;
    cmd.serial_command = "open".to_string();
    cmd.serial_port_name = serial_port;

    send_to_turtlebot(cmd);
    Ok(())
}

pub fn close_port_command() -> Result<()> {
    let mut cmd = Command::new();
    cmd.ty = CommandId::SerialControl;
    cmd.serial_command = "close".to_string();

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

    let mut cmd = Command::new();
    cmd.ty = CommandId::BaseControl;
    cmd.payload = payload;

    send_to_turtlebot(cmd);
    Ok(())
}

pub fn sound_command(freq: u8, amp: u8, duration: u8) -> Result<()> {
    // To avoid divide by zero
    if freq == 0 || amp == 0 || duration == 0 {
        return Err(anyhow!(""));
    }

    let mut payload: Vec<u8> = Vec::new();
    let tmp: u16 = (1 / (freq * amp)) as u16;

    let mut payload: Vec<u8> = Vec::new();
    payload.push(0xaa);
    payload.push(0x55);
    payload.push(CMD_LEN_SOUND);
    payload.push(CommandId::Sound as u8);
    payload.push(CMD_SIZE_SOUND);
    payload.push((tmp & 0xff) as u8);
    payload.push((tmp & 0xff00).shr(8) as u8);
    payload.push(duration);

    let mut cmd = Command::new();
    cmd.ty = CommandId::Sound;
    cmd.payload = payload;

    send_to_turtlebot(cmd);
    Ok(())
}

pub fn sound_sequence_command(seq: u8) -> Result<()> {
    let mut payload: Vec<u8> = Vec::new();
    payload.push(0xaa);
    payload.push(0x55);
    payload.push(CMD_LEN_SOUND_SEQUENCE);
    payload.push(CommandId::SoundSequence as u8);
    payload.push(CMD_SIZE_SOUND_SEQUENCE);
    payload.push(seq);

    let mut cmd = Command::new();
    cmd.ty = CommandId::SoundSequence;
    cmd.payload = payload;

    send_to_turtlebot(cmd);
    Ok(())
}

pub fn request_extra_command(hw_ver: bool, fw_ver: bool, udid: bool) -> Result<()> {
    let mut tmp: u8 = 0;
    tmp |= hw_ver as u8;
    tmp |= (fw_ver as u8).shl(1);
    tmp |= (udid as u8).shl(7);

    let mut payload: Vec<u8> = Vec::new();
    payload.push(0xaa);
    payload.push(0x55);
    payload.push(CMD_LEN_REQUEST_EXTRA);
    payload.push(CommandId::RequestExtra as u8);
    payload.push(CMD_SIZE_REQUEST_EXTRA);
    payload.push(tmp);

    let mut cmd = Command::new();
    cmd.ty = CommandId::RequestExtra;
    cmd.payload = payload;

    send_to_turtlebot(cmd);

    Ok(())
}

pub fn general_purpose_output_command(
    d_out_ch0: bool,
    d_out_ch1: bool,
    d_out_ch2: bool,
    d_out_ch3: bool,
    power_3v3: bool,
    power_5v0: bool,
    power_12v5a: bool,
    power_12v1a5: bool,
    red_led1: bool,
    red_led2: bool,
    green_led1: bool,
    green_led2: bool,
) -> Result<()> {
    let mut tmp0: u8 = 0;
    tmp0 |= d_out_ch0 as u8;
    tmp0 |= (d_out_ch1 as u8).shl(1);
    tmp0 |= (d_out_ch2 as u8).shl(2);
    tmp0 |= (d_out_ch3 as u8).shl(3);
    tmp0 |= (power_3v3 as u8).shl(4);
    tmp0 |= (power_5v0 as u8).shl(5);
    tmp0 |= (power_12v5a as u8).shl(6);
    tmp0 |= (power_12v1a5 as u8).shl(7);
    let mut tmp1: u8 = 0;
    tmp1 |= red_led1 as u8;
    tmp1 |= (green_led1 as u8).shl(1);
    tmp1 |= (red_led2 as u8).shl(2);
    tmp1 |= (green_led2 as u8).shl(3);

    let mut payload: Vec<u8> = Vec::new();
    payload.push(0xaa);
    payload.push(0x55);
    payload.push(CMD_LEN_REQUEST_EXTRA);
    payload.push(CommandId::RequestExtra as u8);
    payload.push(CMD_SIZE_REQUEST_EXTRA);
    payload.push(tmp0);
    payload.push(tmp1);

    let mut cmd = Command::new();
    cmd.ty = CommandId::GeneralPurposeOutput;
    cmd.payload = payload;

    send_to_turtlebot(cmd);

    Ok(())
}

pub fn set_controller_gain_command(is_user_configured: bool, p: u32, i: f32, d: u32) -> Result<()> {
    let mut pp = if p == 0 { 1000 } else { p * 1000 };
    let mut ii = if i < 0.1 || i > 32000.0 {
        (0.1 * 1000.0) as u32
    } else {
        (i * 1000.0) as u32
    };
    let mut dd = if d == 0 { 2 * 1000 } else { d * 1000 };

    let mut payload: Vec<u8> = Vec::new();
    payload.push(0xaa);
    payload.push(0x55);
    payload.push(CMD_LEN_SET_CONTROLLER_GAIN);
    payload.push(CommandId::SetControllerGain as u8);
    payload.push(CMD_SIZE_SET_CONTROLLER_GAIN);
    payload.push(is_user_configured as u8);
    payload.push((pp & 0x000000ff) as u8);
    payload.push((pp & 0x0000ff00).shr(8) as u8);
    payload.push((pp & 0x00ff0000).shr(16) as u8);
    payload.push((pp & 0xff000000).shr(24) as u8);
    payload.push((ii & 0x000000ff) as u8);
    payload.push((ii & 0x0000ff00).shr(8) as u8);
    payload.push((ii & 0x00ff0000).shr(16) as u8);
    payload.push((ii & 0xff000000).shr(24) as u8);
    payload.push((dd & 0x000000ff) as u8);
    payload.push((dd & 0x0000ff00).shr(8) as u8);
    payload.push((dd & 0x00ff0000).shr(16) as u8);
    payload.push((dd & 0xff000000).shr(24) as u8);

    let mut cmd = Command::new();
    cmd.ty = CommandId::SetControllerGain;
    cmd.payload = payload;

    send_to_turtlebot(cmd);

    Ok(())
}

pub fn get_controller_gain() -> Result<()> {
    let mut payload: Vec<u8> = Vec::new();
    payload.push(0xaa);
    payload.push(0x55);
    payload.push(CMD_LEN_GET_CONTROLLER_GAIN);
    payload.push(CommandId::GetControllerGain as u8);
    payload.push(CMD_SIZE_GET_CONTROLLER_GAIN);
    payload.push(0xff);

    let mut cmd = Command::new();
    cmd.ty = CommandId::GetControllerGain;
    cmd.payload = payload;

    send_to_turtlebot(cmd);

    Ok(())
}
