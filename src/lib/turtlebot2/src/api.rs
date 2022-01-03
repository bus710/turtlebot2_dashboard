#![allow(unused)]

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

use crate::parser::*;

pub fn hello2() -> Result<()> {
    eprintln!("{:?}", "hello2");
    parse();

    Ok(())
}

pub fn enum_ports() -> Result<Vec<SerialPortInfo>> {
    let ports = serialport::available_ports().expect("No ports found!");
    Ok(ports)
}

pub fn open_port(port_name: String) {
    eprintln!("{:?}", port_name);

    let mut buffer = [0; 4096];
    let mut port = serialport::new(port_name, 115200)
        .open()
        .expect("Open port");
    port.set_timeout(Duration::from_millis(1024));

    let mut residue = Vec::new();
    for i in 0..3 {
        let len = port.read(&mut buffer).expect("Read failed");
        if len < 70 {
            eprintln!("not enough - {:?}", len);
            thread::sleep(Duration::from_millis(256));
            continue;
        }

        if residue.len() != 0 {
            let tmp = merge_residue(&residue, &buffer[..len]).expect("");
            buffer = tmp.as_slice().try_into().expect("Too many");

            eprintln!("residue found");
        }

        eprintln!("total - {:?} / {:?}", len, &buffer[..len]);
        eprintln!();

        // search for the preambles (0xaa, 0x55)
        let h = search_header(&buffer[..len]).expect("Headers not found");
        eprintln!("header indexes - {:?}", h);
        eprintln!();

        // divide packets by header found
        let packets = divide_packet(&buffer[..len], &h).expect("Packets not found");
        for (i, p) in packets.iter().enumerate() {
            eprintln!("p - {:?}/{:?}", i, p.as_slice());
            // check CRC and set the residue to pass to next iteration.
            let correct_crc = check_crc(&p.clone());
            if !correct_crc {
                eprintln!("CRC not matched");
                residue = p.clone();
            } else {
                residue = Vec::new();
            }
        }
        eprintln!();

        // sleep
        thread::sleep(Duration::from_millis(256)); // with 256 ms, the read returns about 1024 bytes

        eprintln!("==================");
        eprintln!();
    }
}

pub fn merge_residue(residue: &[u8], buffer: &[u8]) -> Result<Vec<u8>> {
    let mut a = residue.clone().to_vec();
    let b = buffer.to_vec();
    a.extend(b);
    Ok(a)
}

pub fn search_header(buffer: &[u8]) -> Result<Vec<usize>> {
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

pub fn divide_packet(buffer: &[u8], h: &[usize]) -> Result<Vec<Vec<u8>>> {
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
            end = buffer.len(); // until the end
        }
        eprintln!("s/e - {:?}/{:?}", start, end);
        let b = buffer[start..end].to_vec().clone();
        p.push(b);
    }
    eprintln!();
    Ok(p)
}

pub fn check_crc(buffer: &Vec<u8>) -> bool {
    let last = buffer.len() - 1;
    let checksum = buffer.as_slice()[last];
    let mut acc: u8 = 0;

    for (i, c) in buffer.iter().enumerate().skip(2) {
        if i == last {
            break;
        }
        acc = acc ^ *c;
    }
    acc == checksum
}
