#![allow(unused)]

use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Error, Result};
use crossbeam_channel as crossbeam;
use flutter_rust_bridge::StreamSink;
use once_cell::sync::OnceCell;
use serialport::{SerialPort, UsbPortInfo};

use crate::api::*;
use crate::tx::*;

// // Variant enum
// #[derive(Debug, Clone, PartialEq)]
// pub enum CommandId {
//     SerialControl = 0, // Only to open/close serial port
//     BaseControl = 1,
//     Sound = 3,
//     SoundSequence = 4,
//     RequestExtra = 9,
//     GeneralPurposeOutput = 12,
//     SetControllerGain = 13,
//     GetControllerGain = 14,
// }

// // These can be used to set the length of command
// // Total length = ID + Size + Payload + CRC
// pub const CMD_LEN_BASE_CONTROL: u8 = 7;
// pub const CMD_LEN_SOUND: u8 = 6;
// pub const CMD_LEN_SOUND_SEQUENCE: u8 = 4;
// pub const CMD_LEN_REQUEST_EXTRA: u8 = 5;
// pub const CMD_LEN_GENERAL_PURPOSE_OUTPUT: u8 = 5;
// pub const CMD_LEN_SET_CONTROLLER_GAIN: u8 = 16;
// pub const CMD_LEN_GET_CONTROLLER_GAIN: u8 = 4;

// // These can be used to set the size of payload
// pub const CMD_SIZE_BASE_CONTROL: u8 = 4;
// pub const CMD_SIZE_SOUND: u8 = 3;
// pub const CMD_SIZE_SOUND_SEQUENCE: u8 = 1;
// pub const CMD_SIZE_REQUEST_EXTRA: u8 = 2;
// pub const CMD_SIZE_GENERAL_PURPOSE_OUTPUT: u8 = 2;
// pub const CMD_SIZE_SET_CONTROLLER_GAIN: u8 = 13;
// pub const CMD_SIZE_GET_CONTROLLER_GAIN: u8 = 1;

// //
// #[derive(Debug, Clone)]
// pub struct Command {
//     pub ty: CommandId,
//     pub serial_command: String,
//     pub serial_port_name: String,
//     pub payload: Vec<u8>,
// }

// // decode (buffer => packets => feedbacks)
// pub fn decode(buffer: &[u8], mut residue: &[u8]) -> Result<(Vec<Feedback>, Vec<u8>)> {
//     // Check if the length if enough.
//     // The min length is 70.
//     let len = buffer.len();
//     if len < 81 {
//         return Err(anyhow!("Not enough data"));
//     }

//     let mut feedbacks = Vec::new();
//     let mut new_residue = Vec::new();

//     // Search for the preambles (0xaa, 0x55)
//     let headers = search_header(&buffer[..len])?; // Headers not found

//     // If the first preambles set is not located at 0 of the buffer, there is a broken packet
//     // The residue from previous iteration and the broken packet
//     // should be used to make a complete packet
//     if headers[0] != 0 && residue.len() != 0 {
//         let broken_packet = &buffer[..headers[0]];
//         let merged_packet = merge_residue(&residue, broken_packet)?; // Merged failed
//         let correct_crc = check_crc(&merged_packet);
//         if correct_crc {
//             let f = format_feedback(&merged_packet)?; // Formatting failed
//             feedbacks.push(f);
//         }
//     }

//     // Divide packets by header found
//     let packets = divide_packet(&buffer[..len], &headers)?; // Packets not found
//     for (i, packet) in packets.iter().enumerate() {
//         // Check CRC and set the residue to pass to next iteration.
//         let correct_crc = check_crc(&packet.clone());
//         if correct_crc {
//             let f = format_feedback(packet)?; // Formatting failed
//             feedbacks.push(f);
//             new_residue = [0u8; 0].to_vec(); // Clear so don't pass to the next iteration
//         } else {
//             new_residue = packet.clone(); // Pass to the next iteration
//         }
//     }
//     return Ok((feedbacks, new_residue));
// }

// fn search_header(buffer: &[u8]) -> Result<Vec<usize>> {
//     let mut h = Vec::new();
//     let buf = buffer.iter();
//     // Need to skip the first byte since this loop accesses [index-1]
//     for (i, c) in buf.enumerate().skip(1) {
//         if buffer[i - 1] == 0xaa && buffer[i] == 0x55 {
//             h.push(i - 1);
//         }
//     }
//     Ok(h)
// }

// fn merge_residue(residue: &[u8], broken_packet: &[u8]) -> Result<Vec<u8>> {
//     let mut a = residue.clone().to_vec();
//     let b = broken_packet.to_vec();
//     a.extend(b);
//     Ok(a)
// }

// fn divide_packet(buffer: &[u8], headers: &[usize]) -> Result<Vec<Vec<u8>>> {
//     let mut packets = Vec::new();
//     let mut start = 0;
//     let mut end = 0;
//     for (i, c) in headers.iter().enumerate() {
//         // Use the indixes from h
//         // Or use bytes until the end if currently last part
//         if i + 1 != headers.len() {
//             start = headers[i];
//             end = headers[i + 1];
//         } else {
//             start = headers[i];
//             end = buffer.len(); // Until the end => residue
//         }
//         let b = buffer[start..end].to_vec().clone();
//         packets.push(b);
//     }
//     Ok(packets)
// }

// fn check_crc(packet: &Vec<u8>) -> bool {
//     // Don't even have preambles and total length.
//     let packet_len = packet.len();
//     if packet_len < 3 {
//         return false;
//     }

//     // if given packet length is NOT same as
//     // the total_len + preambles (2) + length byte (1) + checksum byte (1),
//     // it is broken packet
//     let total_len = packet[2].clone() as usize;
//     if packet_len != total_len + 4 {
//         return false;
//     }

//     let last = packet.len() - 1;
//     let checksum = packet.as_slice()[last];
//     let mut acc: u8 = 0;

//     for (i, c) in packet.iter().enumerate().skip(2) {
//         if i == last {
//             break;
//         }
//         acc = acc ^ c;
//     }
//     acc == checksum
// }

// fn format_feedback(packet: &Vec<u8>) -> Result<Feedback> {
//     let total_len = packet[2].clone();
//     let mut exit_count = 0;
//     let mut index: u8 = 3; // assign the index of first ID of a feedback
//     let mut f = Feedback::new();

//     f.epoch_time_stamp = get_epoch_ms();

//     loop {
//         // There are only 11 entries in FeedBackId.
//         exit_count += 1;
//         if exit_count > 11 {
//             break;
//         }
//         // To prevent Our of Range access
//         if index >= total_len {
//             break;
//         }

//         let id = num::FromPrimitive::from_u8(packet[index as usize]);
//         match id {
//             Some(FeedbackId::BasicSensor) => {
//                 f.basic_sensor.valid = true;
//                 f.basic_sensor.time_stamp = packet[2 + index as usize] as u32;
//                 f.basic_sensor.time_stamp |= (packet[3 + index as usize] as u32) << 8;
//                 f.basic_sensor.bumper = packet[4 + index as usize] as u32;
//                 f.basic_sensor.wheel_drop = packet[5 + index as usize] as u32;
//                 f.basic_sensor.cliff = packet[6 + index as usize] as u32;
//                 f.basic_sensor.left_encoder = packet[7 + index as usize] as u32;
//                 f.basic_sensor.left_encoder |= (packet[8 + index as usize] as u32) << 8;
//                 f.basic_sensor.right_encoder = packet[9 + index as usize] as u32;
//                 f.basic_sensor.right_encoder |= (packet[10 + index as usize] as u32) << 8;
//                 f.basic_sensor.left_pwm = packet[11 + index as usize] as u32;
//                 f.basic_sensor.right_pwm = packet[12 + index as usize] as u32;
//                 f.basic_sensor.button = packet[13 + index as usize] as u32;
//                 f.basic_sensor.charger = packet[14 + index as usize] as u32;
//                 f.basic_sensor.battery = packet[15 + index as usize] as u32;
//                 f.basic_sensor.overcurrent_flags = packet[16 + index as usize] as u32;
//                 index += FDB_SIZE_BASIC_SENSOR_DATA + 2;
//             }
//             Some(FeedbackId::DockingIR) => {
//                 f.docking_ir.valid = true;
//                 f.docking_ir.right_signal = packet[2 + index as usize] as u32;
//                 f.docking_ir.central_signal = packet[3 + index as usize] as u32;
//                 f.docking_ir.left_signal = packet[4 + index as usize] as u32;
//                 index += FDB_SIZE_DOCKING_IR + 2;
//             }
//             Some(FeedbackId::InertialSensor) => {
//                 f.inertial_sensor.valid = true;
//                 f.inertial_sensor.angle = packet[2 + index as usize] as u32;
//                 f.inertial_sensor.angle |= (packet[3 + index as usize] as u32) << 8;
//                 f.inertial_sensor.angle_rate = packet[4 + index as usize] as u32;
//                 f.inertial_sensor.angle_rate |= (packet[5 + index as usize] as u32) << 8;
//                 index += FDB_SIZE_INERTIAL_SENSOR + 2;
//             }
//             Some(FeedbackId::Cliff) => {
//                 f.cliff.valid = true;
//                 f.cliff.right_cliff_sensor = packet[2 + index as usize] as u32;
//                 f.cliff.right_cliff_sensor |= (packet[3 + index as usize] as u32) << 8;
//                 f.cliff.central_cliff_sensor = packet[4 + index as usize] as u32;
//                 f.cliff.central_cliff_sensor |= (packet[5 + index as usize] as u32) << 8;
//                 f.cliff.left_cliff_sensor = packet[6 + index as usize] as u32;
//                 f.cliff.left_cliff_sensor |= (packet[7 + index as usize] as u32) << 8;
//                 index += FDB_SIZE_CLIFF + 2;
//             }
//             Some(FeedbackId::Current) => {
//                 f.current.valid = true;
//                 f.current.left_motor = packet[2 + index as usize] as u32;
//                 f.current.right_motor = packet[3 + index as usize] as u32;
//                 index += FDB_SIZE_CURRENT + 2;
//             }
//             Some(FeedbackId::HardwareVersion) => {
//                 f.hardware_version.valid = true;
//                 f.hardware_version.patch = packet[2 + index as usize] as u32;
//                 f.hardware_version.minor = packet[3 + index as usize] as u32;
//                 f.hardware_version.major = packet[4 + index as usize] as u32;
//                 index += FDB_SIZE_HARDWARE_VERSION + 2;
//             }
//             Some(FeedbackId::FirmwareVersion) => {
//                 f.firmware_version.valid = true;
//                 f.firmware_version.patch = packet[2 + index as usize] as u32;
//                 f.firmware_version.minor = packet[3 + index as usize] as u32;
//                 f.firmware_version.major = packet[4 + index as usize] as u32;
//                 index += FDB_SIZE_FIRMWARE_VERSION + 2;
//             }
//             Some(FeedbackId::RawDataOf3AxisGyro) => {
//                 f.gyro.valid = true;
//                 f.gyro.frame_id = packet[2 + index as usize] as u32;
//                 f.gyro.followed_data_length = packet[3 + index as usize] as u32;
//                 //
//                 f.gyro.x0 = packet[4 + index as usize] as u32;
//                 f.gyro.x0 |= (packet[5 + index as usize] as u32) << 8;
//                 f.gyro.y0 = packet[6 + index as usize] as u32;
//                 f.gyro.y0 |= (packet[7 + index as usize] as u32) << 8;
//                 f.gyro.z0 = packet[8 + index as usize] as u32;
//                 f.gyro.z0 |= (packet[9 + index as usize] as u32) << 8;
//                 //
//                 f.gyro.x1 = packet[4 + index as usize] as u32;
//                 f.gyro.x1 |= (packet[5 + index as usize] as u32) << 8;
//                 f.gyro.y1 = packet[6 + index as usize] as u32;
//                 f.gyro.y1 |= (packet[7 + index as usize] as u32) << 8;
//                 f.gyro.z1 = packet[8 + index as usize] as u32;
//                 f.gyro.z1 |= (packet[9 + index as usize] as u32) << 8;
//                 //
//                 if packet[1 + index as usize] == FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A {
//                     index += FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A + 2;
//                 } else if packet[1 + index as usize] == FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B {
//                     f.gyro.x2 = packet[4 + index as usize] as u32;
//                     f.gyro.x2 |= (packet[5 + index as usize] as u32) << 8;
//                     f.gyro.y2 = packet[6 + index as usize] as u32;
//                     f.gyro.y2 |= (packet[7 + index as usize] as u32) << 8;
//                     f.gyro.z2 = packet[8 + index as usize] as u32;
//                     f.gyro.z2 |= (packet[9 + index as usize] as u32) << 8;
//                     index += FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B + 2;
//                 }
//             }
//             Some(FeedbackId::GeneralPurposeInput) => {
//                 f.general_purpose_input.valid = true;
//                 f.general_purpose_input.d_ch0 = packet[2 + index as usize] as u32;
//                 f.general_purpose_input.d_ch0 |= (packet[3 + index as usize] as u32) << 8;
//                 //
//                 f.general_purpose_input.a_ch0 = packet[4 + index as usize] as u32;
//                 f.general_purpose_input.a_ch0 |= (packet[5 + index as usize] as u32) << 8;
//                 //
//                 f.general_purpose_input.a_ch1 = packet[6 + index as usize] as u32;
//                 f.general_purpose_input.a_ch1 |= (packet[7 + index as usize] as u32) << 8;
//                 //
//                 f.general_purpose_input.a_ch2 = packet[8 + index as usize] as u32;
//                 f.general_purpose_input.a_ch2 |= (packet[9 + index as usize] as u32) << 8;
//                 //
//                 f.general_purpose_input.a_ch3 = packet[10 + index as usize] as u32;
//                 f.general_purpose_input.a_ch3 |= (packet[11 + index as usize] as u32) << 8;
//                 index += FDB_SIZE_GENERAL_PURPOSE_OUTPUT + 2;
//             }
//             Some(FeedbackId::UniqueDeviceId) => {
//                 f.unique_device_id.valid = true;
//                 f.unique_device_id.udid0 = packet[2 + index as usize] as u32;
//                 f.unique_device_id.udid0 |= (packet[3 + index as usize] as u32) << 8;
//                 f.unique_device_id.udid0 |= (packet[4 + index as usize] as u32) << 16;
//                 f.unique_device_id.udid0 |= (packet[5 + index as usize] as u32) << 24;
//                 //
//                 f.unique_device_id.udid1 = packet[6 + index as usize] as u32;
//                 f.unique_device_id.udid1 |= (packet[7 + index as usize] as u32) << 8;
//                 f.unique_device_id.udid1 |= (packet[8 + index as usize] as u32) << 16;
//                 f.unique_device_id.udid1 |= (packet[9 + index as usize] as u32) << 24;
//                 //
//                 f.unique_device_id.udid2 = packet[10 + index as usize] as u32;
//                 f.unique_device_id.udid2 |= (packet[11 + index as usize] as u32) << 8;
//                 f.unique_device_id.udid2 |= (packet[12 + index as usize] as u32) << 16;
//                 f.unique_device_id.udid2 |= (packet[13 + index as usize] as u32) << 24;
//                 index += FDB_SIZE_UNIQUE_DEVICE_IDENTIFIER + 2;
//             }
//             Some(FeedbackId::ControllerInfo) => {
//                 f.controller_info.valid = true;
//                 f.controller_info.p_gain = packet[2 + index as usize] as u32;
//                 f.controller_info.p_gain |= (packet[3 + index as usize] as u32) << 8;
//                 f.controller_info.p_gain |= (packet[4 + index as usize] as u32) << 16;
//                 f.controller_info.p_gain |= (packet[5 + index as usize] as u32) << 24;
//                 //
//                 f.controller_info.i_gain = packet[6 + index as usize] as u32;
//                 f.controller_info.i_gain |= (packet[7 + index as usize] as u32) << 8;
//                 f.controller_info.i_gain |= (packet[8 + index as usize] as u32) << 16;
//                 f.controller_info.i_gain |= (packet[9 + index as usize] as u32) << 24;
//                 //
//                 f.controller_info.d_gain = packet[10 + index as usize] as u32;
//                 f.controller_info.d_gain |= (packet[11 + index as usize] as u32) << 8;
//                 f.controller_info.d_gain |= (packet[12 + index as usize] as u32) << 16;
//                 f.controller_info.d_gain |= (packet[13 + index as usize] as u32) << 24;
//                 index += FDB_SIZE_CONTROLLER_INFO + 2;
//             }
//             _ => {
//                 // Nothing to do
//             }
//         }
//     }
//     Ok(f)
// }

// // https://stackoverflow.com/a/65051530
// fn get_epoch_ms() -> String {
//     SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_millis()
//         .to_string()
// }

// Static channel to interact with turtlebot
static SEND: OnceCell<Arc<Mutex<crossbeam::Sender<Command>>>> = OnceCell::new();
// Static vector to store feedbacks from turtlebot
static RECEIVE: OnceCell<Arc<Mutex<Vec<Feedback>>>> = OnceCell::new();

pub fn set_statics_in_turtlebot(sender: crossbeam::Sender<Command>) {
    // The global static SEND is used to send command to the turtlebot instance
    let sender_lock = Arc::new(Mutex::new(sender));
    SEND.set(sender_lock);
}

pub fn send(cmd: Command) {
    let tx_lock = SEND.get().unwrap();
    let tx = tx_lock.lock().unwrap();
    tx.send(cmd);
}

pub fn receive() -> Result<Vec<Feedback>> {
    let fbd_lock = RECEIVE.get().unwrap();
    let fbd = fbd_lock.lock().unwrap();
    if fbd.len() > 0 {
        RECEIVE.set(Arc::new(Mutex::new(Vec::new())));
        return Ok(fbd.clone());
    }
    Err(anyhow!("What feedback?"))
}

// TurtlebotData runs the serial thread and passes commands/feedbacks
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
            // To interact with outside
            receiver: rx,
            sink: sk,
            feedbacks: Vec::new(),
            // Serial port state indicators
            current_port_opened: false,
            current_port_name: "".to_string(),
            // Be careful! - these channels are twisted.
            ttb_tx: tx1,
            serial_rx: rx1,
            ttb_rx: rx2,
            serial_tx: tx2,
        }
    }

    pub fn serial_runner(
        &mut self,
        cmd: Command,
        tx: crossbeam::Sender<Command>,
        rx: crossbeam::Receiver<Command>,
    ) {
        thread::spawn(move || {
            // Ticker to periodically read a port if opened
            let ticker = crossbeam::tick(Duration::from_millis(64));
            let serial_port_name = cmd.serial_port_name.clone();
            let port_b = serialport::new(serial_port_name.clone(), 115_200).open();
            match port_b {
                Ok(mut p) => {
                    // Need to send
                    tx.send(Command {
                        ty: CommandId::SerialControl,
                        serial_command: "opened".to_string(),
                        serial_port_name: serial_port_name.clone(),
                        payload: Vec::new(),
                    });

                    let mut buffer = [0; 4096];
                    let mut residue = Vec::new();

                    loop {
                        crossbeam::select! {
                            recv(rx) -> cmd => {
                                let c = cmd.unwrap();
                                if c.serial_command == "close" {
                                    tx.send(Command {
                                        ty: CommandId::SerialControl,
                                        serial_command: "closed".to_string(),
                                        serial_port_name: serial_port_name.clone(),
                                        payload: Vec::new(),
                                    });
                                    break;
                                }
                            }
                            recv(ticker)-> _ => {
                                // ttb_data.sink.add("a".to_string());
                                // thread::sleep(Duration::from_millis(10));
                                let len = p.read(&mut buffer).expect("Read failed");
                                let d = decode(&buffer[..len], &residue);
                                match d {
                                    Ok(v) => {
                                        let(mut f, r) = v;
                                        let fdb_lock = RECEIVE.get().unwrap();
                                        let mut fdb = fdb_lock.lock().unwrap();
                                        fdb.append(&mut f);
                                        let fdb_locak = Arc::new(Mutex::new(fdb));
                                        RECEIVE.set(fdb_lock.clone());
                                        residue = r;

                                        tx.send(Command {
                                            ty: CommandId::SerialControl,
                                            serial_command: "ready".to_string(),
                                            serial_port_name: "".to_string(),
                                            payload: Vec::new(),
                                        });

                                    }
                                    Err(e) => {
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            }
        });
    }
}

// Turtlebot handles the communication between Flutter and serial device
pub struct Turtlebot {
    turtlebot_lock: Arc<Mutex<TurtlebotData>>,
}

impl Turtlebot {
    pub fn new(rx: crossbeam::Receiver<Command>, sk: StreamSink<String>) -> Turtlebot {
        //
        let feedback_lock = Arc::new(Mutex::new(Vec::new()));
        RECEIVE.set(feedback_lock);
        //
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
                    // From Flutter => the serial thread
                    recv(ttb_data.receiver) -> cmd =>{
                        let cmd = cmd.unwrap();
                        eprintln!("received");
                        match cmd.ty {
                            CommandId::SerialControl => {
                                if cmd.serial_command == "open" && !ttb_data.current_port_opened {
                                    let cmd = cmd.clone();
                                    let tx = ttb_data.serial_tx.clone();
                                    let rx = ttb_data.serial_rx.clone();
                                    ttb_data.serial_runner(cmd, tx, rx);
                                }
                            }
                            // All non-SerialControl commands
                            _ => {
                                ttb_data.ttb_tx.send(cmd);
                            }
                        }
                    }
                    // From the serial thread => Flutter
                    recv(ttb_data.ttb_rx) -> cmd =>{
                        let c = cmd.unwrap();
                        if c.serial_command == "opened" {
                            ttb_data.current_port_name= c.serial_port_name;
                            ttb_data.current_port_opened = true;
                        }
                        if c.serial_command == "closed" {
                            ttb_data.current_port_name= "".to_string();
                            ttb_data.current_port_opened = false;
                        }
                        if c.serial_command == "ready" {
                            ttb_data.sink.add("ready".to_string());
                        }
                    }
                }
            }
        });
    }
}
