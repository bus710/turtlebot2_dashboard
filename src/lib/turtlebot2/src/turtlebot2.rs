#![allow(unused)]

use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Error, Result};
use crossbeam_channel as crossbeam;
use flutter_rust_bridge::StreamSink;
use serialport::{SerialPort, UsbPortInfo};

use crate::api::*;

// Variant enum
#[derive(Debug, Clone, PartialEq)]
pub enum CommandId {
    SerialControl = 0, // Only to open/close serial port
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

// decode (buffer => packets => feedbacks)
pub fn decode(buffer: &[u8], mut residue: &[u8]) -> Result<(Vec<Feedback>, Vec<u8>)> {
    // Check if the length if enough.
    // The min length is 70.
    let len = buffer.len();
    if len < 81 {
        return Err(anyhow!("Not enough data"));
    }

    let mut feedbacks = Vec::new();
    let mut new_residue = Vec::new();

    // Search for the preambles (0xaa, 0x55)
    let headers = search_header(&buffer[..len])?; // Headers not found

    // If the first preambles set is not located at 0 of the buffer, there is a broken packet
    // The residue from previous iteration and the broken packet
    // should be used to make a complete packet
    if headers[0] != 0 && residue.len() != 0 {
        let broken_packet = &buffer[..headers[0]];
        let merged_packet = merge_residue(&residue, broken_packet)?; // Merged failed
        let correct_crc = check_crc(&merged_packet);
        if correct_crc {
            let f = format_feedback(&merged_packet)?; // Formatting failed
            feedbacks.push(f);
        }
    }

    // Divide packets by header found
    let packets = divide_packet(&buffer[..len], &headers)?; // Packets not found
    for (i, packet) in packets.iter().enumerate() {
        // Check CRC and set the residue to pass to next iteration.
        let correct_crc = check_crc(&packet.clone());
        if correct_crc {
            let f = format_feedback(packet)?; // Formatting failed
            feedbacks.push(f);
            new_residue = [0u8; 0].to_vec(); // Clear so don't pass to the next iteration
        } else {
            new_residue = packet.clone(); // Pass to the next iteration
        }
    }
    return Ok((feedbacks, new_residue));
}

fn search_header(buffer: &[u8]) -> Result<Vec<usize>> {
    let mut h = Vec::new();
    let buf = buffer.iter();
    // Need to skip the first byte since this loop accesses [index-1]
    for (i, c) in buf.enumerate().skip(1) {
        if buffer[i - 1] == 0xaa && buffer[i] == 0x55 {
            h.push(i - 1);
        }
    }
    Ok(h)
}

fn merge_residue(residue: &[u8], broken_packet: &[u8]) -> Result<Vec<u8>> {
    let mut a = residue.clone().to_vec();
    let b = broken_packet.to_vec();
    a.extend(b);
    Ok(a)
}

fn divide_packet(buffer: &[u8], headers: &[usize]) -> Result<Vec<Vec<u8>>> {
    let mut packets = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for (i, c) in headers.iter().enumerate() {
        // Use the indixes from h
        // Or use bytes until the end if currently last part
        if i + 1 != headers.len() {
            start = headers[i];
            end = headers[i + 1];
        } else {
            start = headers[i];
            end = buffer.len(); // Until the end => residue
        }
        let b = buffer[start..end].to_vec().clone();
        packets.push(b);
    }
    Ok(packets)
}

fn check_crc(packet: &Vec<u8>) -> bool {
    // Don't even have preambles and total length.
    let packet_len = packet.len();
    if packet_len < 3 {
        return false;
    }

    // if given packet length is NOT same as
    // the total_len + preambles (2) + length byte (1) + checksum byte (1),
    // it is broken packet
    let total_len = packet[2].clone() as usize;
    if packet_len != total_len + 4 {
        return false;
    }

    let last = packet.len() - 1;
    let checksum = packet.as_slice()[last];
    let mut acc: u8 = 0;

    for (i, c) in packet.iter().enumerate().skip(2) {
        if i == last {
            break;
        }
        acc = acc ^ c;
    }
    acc == checksum
}

fn format_feedback(packet: &Vec<u8>) -> Result<Feedback> {
    let total_len = packet[2].clone();
    let mut exit_count = 0;
    let mut index: u8 = 3; // assign the index of first ID of a feedback
    let mut f = Feedback::new();

    f.epoch_time_stamp = get_epoch_ms();

    loop {
        // There are only 11 entries in FeedBackId.
        exit_count += 1;
        if exit_count > 11 {
            break;
        }
        // To prevent Our of Range access
        if index >= total_len {
            break;
        }

        let id = num::FromPrimitive::from_u8(packet[index as usize]);
        match id {
            Some(FeedbackId::BasicSensor) => {
                f.basic_sensor.valid = true;
                f.basic_sensor.time_stamp = packet[2 + index as usize] as u32;
                f.basic_sensor.time_stamp |= (packet[3 + index as usize] as u32) << 8;
                f.basic_sensor.bumper = packet[4 + index as usize] as u32;
                f.basic_sensor.wheel_drop = packet[5 + index as usize] as u32;
                f.basic_sensor.cliff = packet[6 + index as usize] as u32;
                f.basic_sensor.left_encoder = packet[7 + index as usize] as u32;
                f.basic_sensor.left_encoder |= (packet[8 + index as usize] as u32) << 8;
                f.basic_sensor.right_encoder = packet[9 + index as usize] as u32;
                f.basic_sensor.right_encoder |= (packet[10 + index as usize] as u32) << 8;
                f.basic_sensor.left_pwm = packet[11 + index as usize] as u32;
                f.basic_sensor.right_pwm = packet[12 + index as usize] as u32;
                f.basic_sensor.button = packet[13 + index as usize] as u32;
                f.basic_sensor.charger = packet[14 + index as usize] as u32;
                f.basic_sensor.battery = packet[15 + index as usize] as u32;
                f.basic_sensor.overcurrent_flags = packet[16 + index as usize] as u32;
                index += FDB_SIZE_BASIC_SENSOR_DATA + 2;
            }
            Some(FeedbackId::DockingIR) => {
                f.docking_ir.valid = true;
                f.docking_ir.right_signal = packet[2 + index as usize] as u32;
                f.docking_ir.central_signal = packet[3 + index as usize] as u32;
                f.docking_ir.left_signal = packet[4 + index as usize] as u32;
                index += FDB_SIZE_DOCKING_IR + 2;
            }
            Some(FeedbackId::InertialSensor) => {
                f.inertial_sensor.valid = true;
                f.inertial_sensor.angle = packet[2 + index as usize] as u32;
                f.inertial_sensor.angle |= (packet[3 + index as usize] as u32) << 8;
                f.inertial_sensor.angle_rate = packet[4 + index as usize] as u32;
                f.inertial_sensor.angle_rate |= (packet[5 + index as usize] as u32) << 8;
                index += FDB_SIZE_INERTIAL_SENSOR + 2;
            }
            Some(FeedbackId::Cliff) => {
                f.cliff.valid = true;
                f.cliff.right_cliff_sensor = packet[2 + index as usize] as u32;
                f.cliff.right_cliff_sensor |= (packet[3 + index as usize] as u32) << 8;
                f.cliff.central_cliff_sensor = packet[4 + index as usize] as u32;
                f.cliff.central_cliff_sensor |= (packet[5 + index as usize] as u32) << 8;
                f.cliff.left_cliff_sensor = packet[6 + index as usize] as u32;
                f.cliff.left_cliff_sensor |= (packet[7 + index as usize] as u32) << 8;
                index += FDB_SIZE_CLIFF + 2;
            }
            Some(FeedbackId::Current) => {
                f.current.valid = true;
                f.current.left_motor = packet[2 + index as usize] as u32;
                f.current.right_motor = packet[3 + index as usize] as u32;
                index += FDB_SIZE_CURRENT + 2;
            }
            Some(FeedbackId::HardwareVersion) => {
                f.hardware_version.valid = true;
                f.hardware_version.patch = packet[2 + index as usize] as u32;
                f.hardware_version.minor = packet[3 + index as usize] as u32;
                f.hardware_version.major = packet[4 + index as usize] as u32;
                index += FDB_SIZE_HARDWARE_VERSION + 2;
            }
            Some(FeedbackId::FirmwareVersion) => {
                f.firmware_version.valid = true;
                f.firmware_version.patch = packet[2 + index as usize] as u32;
                f.firmware_version.minor = packet[3 + index as usize] as u32;
                f.firmware_version.major = packet[4 + index as usize] as u32;
                index += FDB_SIZE_FIRMWARE_VERSION + 2;
            }
            Some(FeedbackId::RawDataOf3AxisGyro) => {
                f.gyro.valid = true;
                f.gyro.frame_id = packet[2 + index as usize] as u32;
                f.gyro.followed_data_length = packet[3 + index as usize] as u32;
                //
                f.gyro.x0 = packet[4 + index as usize] as u32;
                f.gyro.x0 |= (packet[5 + index as usize] as u32) << 8;
                f.gyro.y0 = packet[6 + index as usize] as u32;
                f.gyro.y0 |= (packet[7 + index as usize] as u32) << 8;
                f.gyro.z0 = packet[8 + index as usize] as u32;
                f.gyro.z0 |= (packet[9 + index as usize] as u32) << 8;
                //
                f.gyro.x1 = packet[4 + index as usize] as u32;
                f.gyro.x1 |= (packet[5 + index as usize] as u32) << 8;
                f.gyro.y1 = packet[6 + index as usize] as u32;
                f.gyro.y1 |= (packet[7 + index as usize] as u32) << 8;
                f.gyro.z1 = packet[8 + index as usize] as u32;
                f.gyro.z1 |= (packet[9 + index as usize] as u32) << 8;
                //
                if packet[1 + index as usize] == FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A {
                    index += FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A + 2;
                } else if packet[1 + index as usize] == FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B {
                    f.gyro.x2 = packet[4 + index as usize] as u32;
                    f.gyro.x2 |= (packet[5 + index as usize] as u32) << 8;
                    f.gyro.y2 = packet[6 + index as usize] as u32;
                    f.gyro.y2 |= (packet[7 + index as usize] as u32) << 8;
                    f.gyro.z2 = packet[8 + index as usize] as u32;
                    f.gyro.z2 |= (packet[9 + index as usize] as u32) << 8;
                    index += FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B + 2;
                }
            }
            Some(FeedbackId::GeneralPurposeInput) => {
                f.general_purpose_input.valid = true;
                f.general_purpose_input.d_ch0 = packet[2 + index as usize] as u32;
                f.general_purpose_input.d_ch0 |= (packet[3 + index as usize] as u32) << 8;
                //
                f.general_purpose_input.a_ch0 = packet[4 + index as usize] as u32;
                f.general_purpose_input.a_ch0 |= (packet[5 + index as usize] as u32) << 8;
                //
                f.general_purpose_input.a_ch1 = packet[6 + index as usize] as u32;
                f.general_purpose_input.a_ch1 |= (packet[7 + index as usize] as u32) << 8;
                //
                f.general_purpose_input.a_ch2 = packet[8 + index as usize] as u32;
                f.general_purpose_input.a_ch2 |= (packet[9 + index as usize] as u32) << 8;
                //
                f.general_purpose_input.a_ch3 = packet[10 + index as usize] as u32;
                f.general_purpose_input.a_ch3 |= (packet[11 + index as usize] as u32) << 8;
                index += FDB_SIZE_GENERAL_PURPOSE_OUTPUT + 2;
            }
            Some(FeedbackId::UniqueDeviceId) => {
                f.unique_device_id.valid = true;
                f.unique_device_id.udid0 = packet[2 + index as usize] as u32;
                f.unique_device_id.udid0 |= (packet[3 + index as usize] as u32) << 8;
                f.unique_device_id.udid0 |= (packet[4 + index as usize] as u32) << 16;
                f.unique_device_id.udid0 |= (packet[5 + index as usize] as u32) << 24;
                //
                f.unique_device_id.udid1 = packet[6 + index as usize] as u32;
                f.unique_device_id.udid1 |= (packet[7 + index as usize] as u32) << 8;
                f.unique_device_id.udid1 |= (packet[8 + index as usize] as u32) << 16;
                f.unique_device_id.udid1 |= (packet[9 + index as usize] as u32) << 24;
                //
                f.unique_device_id.udid2 = packet[10 + index as usize] as u32;
                f.unique_device_id.udid2 |= (packet[11 + index as usize] as u32) << 8;
                f.unique_device_id.udid2 |= (packet[12 + index as usize] as u32) << 16;
                f.unique_device_id.udid2 |= (packet[13 + index as usize] as u32) << 24;
                index += FDB_SIZE_UNIQUE_DEVICE_IDENTIFIER + 2;
            }
            Some(FeedbackId::ControllerInfo) => {
                f.controller_info.valid = true;
                f.controller_info.p_gain = packet[2 + index as usize] as u32;
                f.controller_info.p_gain |= (packet[3 + index as usize] as u32) << 8;
                f.controller_info.p_gain |= (packet[4 + index as usize] as u32) << 16;
                f.controller_info.p_gain |= (packet[5 + index as usize] as u32) << 24;
                //
                f.controller_info.i_gain = packet[6 + index as usize] as u32;
                f.controller_info.i_gain |= (packet[7 + index as usize] as u32) << 8;
                f.controller_info.i_gain |= (packet[8 + index as usize] as u32) << 16;
                f.controller_info.i_gain |= (packet[9 + index as usize] as u32) << 24;
                //
                f.controller_info.d_gain = packet[10 + index as usize] as u32;
                f.controller_info.d_gain |= (packet[11 + index as usize] as u32) << 8;
                f.controller_info.d_gain |= (packet[12 + index as usize] as u32) << 16;
                f.controller_info.d_gain |= (packet[13 + index as usize] as u32) << 24;
                index += FDB_SIZE_CONTROLLER_INFO + 2;
            }
            _ => {
                // Nothing to do
            }
        }
    }
    Ok(f)
}

// https://stackoverflow.com/a/65051530
fn get_epoch_ms() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

#[derive(Clone)]
pub struct TurtlebotData {
    receiver: crossbeam::Receiver<Command>,
    sink: StreamSink<String>,
    feedbacks: Vec<Feedback>,
    current_port_opened: bool,
    current_port_name: String,
    ttb_tx: crossbeam::Sender<Command>,
    ttb_rx: crossbeam::Receiver<Command>,
    serial_tx: crossbeam::Sender<Command>,
    serial_rx: crossbeam::Receiver<Command>,
}

impl TurtlebotData {
    pub fn new(rx: crossbeam::Receiver<Command>, sk: StreamSink<String>) -> TurtlebotData {
        let (tx1, rx1) = crossbeam::unbounded();
        let (tx2, rx2) = crossbeam::unbounded();
        TurtlebotData {
            receiver: rx,
            sink: sk,
            feedbacks: Vec::new(),
            current_port_opened: false,
            current_port_name: "".to_string(),
            ttb_tx: tx1,
            serial_rx: rx1,
            ttb_rx: rx2,
            serial_tx: tx2,
        }
    }

    pub fn command_handler(&mut self, cmd: Command) {
        eprintln!("received");
        match cmd.ty {
            CommandId::SerialControl => {
                if cmd.serial_command == "open" && !self.current_port_opened {
                    let tx = self.serial_tx.clone();
                    let rx = self.serial_rx.clone();
                    thread::spawn(move || {
                        let serial_port_name = cmd.serial_port_name.clone();
                        let port_b = serialport::new(serial_port_name.clone(), 115_200).open();
                        // Ticker to read a port if opened
                        let ticker = crossbeam::tick(Duration::from_millis(1000));
                        match port_b {
                            Ok(p) => {
                                // Need to send
                                tx.send(Command {
                                    ty: CommandId::SerialControl,
                                    serial_command: "opened".to_string(),
                                    serial_port_name: serial_port_name.clone(),
                                    payload: Vec::new(),
                                });
                                loop {
                                    crossbeam::select! {
                                        recv(rx) -> cmd => {
                                            let c = cmd.unwrap();
                                            if c.serial_command == "close" {
                                                // eprintln!("close");
                                                // drop(port);
                                                // self.current_port_opened = false;
                                                // self.current_port_name = "".to_string();
                                                break;
                                            }
                                        }
                                        recv(ticker)-> _ => {
                                            eprintln!("tick");
                                            // ttb_data.sink.add("a".to_string());
                                            // thread::sleep(Duration::from_millis(10));
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("{:?}", e);
                            }
                        }
                    });

                    // port.set_timeout(Duration::from_millis(1024));
                    // self.current_port_opened = true;
                    // self.current_port_name = port.name().unwrap().clone();
                }
            }
            _ => {
                self.command_handler(cmd);
            }
        }
    }

    pub fn tick_handler(&mut self) {
        eprintln!("Ticker running");
    }
}

pub struct Turtlebot {
    turtlebot_lock: Arc<Mutex<TurtlebotData>>,
}

impl Turtlebot {
    pub fn new(rx: crossbeam::Receiver<Command>, sk: StreamSink<String>) -> Turtlebot {
        let ttb_data = TurtlebotData::new(rx, sk);
        Turtlebot {
            turtlebot_lock: Arc::new(Mutex::new(ttb_data)),
        }
    }

    pub fn run(&mut self) {
        // Get the mutex
        let ttb_lock = self.turtlebot_lock.clone();
        // Thread body
        thread::spawn(move || {
            // Unlock the mutex
            let mut ttb_data = ttb_lock.lock().unwrap();
            // Enter the loop
            loop {
                crossbeam::select! {
                    // From Flutter
                    recv(ttb_data.receiver) -> cmd =>{
                        let c = cmd.unwrap();
                        ttb_data.command_handler(c);
                    }
                    // From internal thread for serial
                    recv(ttb_data.ttb_rx) -> cmd =>{
                        let c = cmd.unwrap();
                        if c.serial_command == "opened" {
                            ttb_data.current_port_name= c.serial_port_name;
                            ttb_data.current_port_opened = true;
                        }
                    }
                }
            }
        });
    }
}

// pub fn open_port(port_name: String) {
//     eprintln!("{:?}", port_name);

//     let mut port = serialport::new(port_name, 115200)
//         .open()
//         .expect("Open port");
//     port.set_timeout(Duration::from_millis(1024));

//     let mut buffer = [0; 4096];
//     let mut residue = Vec::new();

//     for i in 0..10 {
//         let len = port.read(&mut buffer).expect("Read failed");
//         let d = ttb2::rx::decode(&buffer[..len], &residue);
//         match d {
//             Ok(v) => {
//                 let (f, r) = v;
//                 // eprintln!("f - {:?}", f);
//                 residue = r;
//             }
//             Err(e) => {
//                 eprintln!("Error - {:?}", e);
//             }
//         }
//         eprintln!("================== {:?}", i);
//         thread::sleep(Duration::from_millis(64)); // with 64 ms, the read returns about 220~350 bytes
//     }

//     // let cmd = base_control_command(0x1, 0x1).expect("");
//     // port.write(&cmd);
//     // thread::sleep(Duration::from_millis(1000)); // with 64 ms, the read returns about 220~350 bytes
//     // let cmd = base_control_command(0x0, 0x0).expect("");
//     // port.write(&cmd);
// }
