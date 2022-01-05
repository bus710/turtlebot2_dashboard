#![allow(unused)]

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
                //
                format_feedback(raw_packet);
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

pub fn format_feedback(raw_packet: &Vec<u8>) {
    let total_len = raw_packet[2].clone();
    let index = 3;

    let f = turtlebot2::Feedback::new();
    eprintln!("f - {:?}", f);
}
