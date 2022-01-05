#![allow(unused)]

use std::ops::Shl;
use std::os::raw;
use std::slice::Iter;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{self, mpsc, Arc};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use crossbeam_channel::unbounded;
use itertools::Itertools;
use serialport::SerialPortInfo;

use flutter_rust_bridge::rust2dart::TaskCallback;
use flutter_rust_bridge::{StreamSink, SyncReturn, ZeroCopyBuffer};

use crate::turtlebot2::{self, *};

pub fn hello2() -> Result<()> {
    eprintln!("{:?}", "hello2");
    hello();

    Ok(())
}

pub fn enum_ports() -> Result<Vec<SerialPortInfo>> {
    let ports = serialport::available_ports().expect("No ports found!");
    Ok(ports)
}

pub fn open_port(port_name: String) {
    eprintln!("{:?}", port_name);

    let mut packet = [0; 4096];
    let mut port = serialport::new(port_name, 115200)
        .open()
        .expect("Open port");
    port.set_timeout(Duration::from_millis(1024));

    let mut residue = Vec::new();
    for i in 0..10 {
        let len = port.read(&mut packet).expect("Read failed");
        if len < 70 {
            eprintln!("Not enough - {:?}", len);
            thread::sleep(Duration::from_millis(64));
            continue;
        }

        eprintln!("total - {:?}", len);
        eprintln!();

        // search for the preambles (0xaa, 0x55)
        let headers = search_header(&packet[..len]).expect("Headers not found");
        eprintln!("header indexes - {:?}", headers);
        eprintln!();

        // if the first preambles set is not located at 0 of the buffer,
        // the residue from previous iteration should be used to make a complete packet
        if headers[0] != 0 {
            // eprintln!("residue packet - {:?}", residue);
            // eprintln!("broken packet - {:?}", &buffer[..headers[0]]);
            let broken_packet = &packet[..headers[0]];
            if residue.len() != 0 {
                let tmp = merge_residue(&residue, broken_packet).expect("");
                let correct_crc = check_crc(&tmp);
                eprintln!("residue & broken (crc: {:?}) - {:?}", correct_crc, tmp);
                eprintln!();
                let f = format_feedback(&broken_packet.to_vec()).expect("Wrong format");
                eprintln!("feedback - {:?}", f);
            }
        }

        // divide packets by header found
        let raw_packets = divide_packet(&packet[..len], &headers).expect("Packets not found");
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
                residue = raw_packet.clone(); // pass to the next iteration
            } else {
                let f = format_feedback(raw_packet);
                eprintln!("feedback - {:?}", f);
                residue = Vec::new(); // clear so don't pass to the next iteration
            }
        }
        eprintln!();

        // sleep
        thread::sleep(Duration::from_millis(64)); // with 64 ms, the read returns about 220~350 bytes

        eprintln!("==================");
        eprintln!();
    }
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
    let mut f = turtlebot2::Feedback::new();

    f.epoch_time_stamp = get_epoch_ms();

    // eprintln!("f - {:?}", f);

    loop {
        exit_count += 1;
        if index >= total_len || exit_count > 20 {
            break;
        }
        let id = num::FromPrimitive::from_u8(raw_packet[index as usize]);
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
                index += turtlebot2::FDB_SIZE_BASIC_SENSOR_DATA + 2;
            }
            Some(FeedbackId::DockingIR) => {
                f.docking_ir.valid = true;
                f.docking_ir.right_signal = raw_packet[2 + index as usize];
                f.docking_ir.central_signal = raw_packet[3 + index as usize];
                f.docking_ir.left_signal = raw_packet[4 + index as usize];
                index += turtlebot2::FDB_SIZE_DOCKING_IR + 2;
            }
            Some(FeedbackId::InertialSensor) => {
                f.inertial_sensor.valid = true;
                f.inertial_sensor.angle = raw_packet[2 + index as usize] as u16;
                f.inertial_sensor.angle |= (raw_packet[3 + index as usize] as u16).shl(8);
                f.inertial_sensor.angle_rate = raw_packet[4 + index as usize] as u16;
                f.inertial_sensor.angle_rate |= (raw_packet[5 + index as usize] as u16).shl(8);
                index += turtlebot2::FDB_SIZE_INERTIAL_SENSOR + 2;
            }
            Some(FeedbackId::Cliff) => {
                f.cliff.valid = true;
                f.cliff.right_cliff_sensor = raw_packet[2 + index as usize] as u16;
                f.cliff.right_cliff_sensor |= (raw_packet[3 + index as usize] as u16).shl(8);
                f.cliff.central_cliff_sensor = raw_packet[4 + index as usize] as u16;
                f.cliff.central_cliff_sensor |= (raw_packet[5 + index as usize] as u16).shl(8);
                f.cliff.left_cliff_sensor = raw_packet[6 + index as usize] as u16;
                f.cliff.left_cliff_sensor |= (raw_packet[7 + index as usize] as u16).shl(8);
                index += turtlebot2::FDB_SIZE_CLIFF + 2;
            }
            Some(FeedbackId::Current) => {
                f.current.valid = true;
                f.current.left_motor = raw_packet[2 + index as usize];
                f.current.right_motor = raw_packet[3 + index as usize];
                index += turtlebot2::FDB_SIZE_CURRENT + 2;
            }
            Some(FeedbackId::HardwareVersion) => {
                f.hardware_version.valid = true;
                f.hardware_version.patch = raw_packet[2 + index as usize];
                f.hardware_version.minor = raw_packet[3 + index as usize];
                f.hardware_version.major = raw_packet[4 + index as usize];
                index += turtlebot2::FDB_SIZE_HARDWARE_VERSION + 2;
            }
            Some(FeedbackId::FirmwareVersion) => {
                f.firmware_version.valid = true;
                f.firmware_version.patch = raw_packet[2 + index as usize];
                f.firmware_version.minor = raw_packet[3 + index as usize];
                f.firmware_version.major = raw_packet[4 + index as usize];
                index += turtlebot2::FDB_SIZE_FIRMWARE_VERSION + 2;
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
                if raw_packet[1 + index as usize] == turtlebot2::FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A {
                    index += turtlebot2::FDB_SIZE_RAW_DATA_3_AXIS_GYRO_A + 2;
                } else if raw_packet[1 + index as usize]
                    == turtlebot2::FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B
                {
                    f.gyro.raw_gyro_data[2].x = raw_packet[16 + index as usize] as u16;
                    f.gyro.raw_gyro_data[2].x |= (raw_packet[17 + index as usize] as u16).shl(8);
                    f.gyro.raw_gyro_data[2].y = raw_packet[18 + index as usize] as u16;
                    f.gyro.raw_gyro_data[2].y |= (raw_packet[19 + index as usize] as u16).shl(8);
                    f.gyro.raw_gyro_data[2].z = raw_packet[20 + index as usize] as u16;
                    f.gyro.raw_gyro_data[2].z |= (raw_packet[21 + index as usize] as u16).shl(8);
                    index += turtlebot2::FDB_SIZE_RAW_DATA_3_AXIS_GYRO_B + 2;
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
                index += turtlebot2::FDB_SIZE_GENERAL_PURPOSE_OUTPUT + 2;
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
                index += turtlebot2::FDB_SIZE_UNIQUE_DEVICE_IDENTIFIER + 2;
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
                index += turtlebot2::FDB_SIZE_CONTROLLER_INFO + 2;
            }
            _ => {
            }
        }
    }
    Ok(f)
}
