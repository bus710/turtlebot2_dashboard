#![allow(unused)]

use std::num::NonZeroI32;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{self, mpsc, Arc};
use std::thread;
use std::time::Duration;

use crate::parser::*;
use anyhow::Result;
use crossbeam_channel::unbounded;
use flutter_rust_bridge::rust2dart::TaskCallback;
use flutter_rust_bridge::{StreamSink, SyncReturn, ZeroCopyBuffer};
use serialport::SerialPortInfo;

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

    for i in 0..3 {
        let len = port.read(&mut buffer).expect("Read failed");
        if len < 64 {
            eprintln!("{:?}", len);
            thread::sleep(Duration::from_millis(256));
            continue;
        }
        eprintln!("{:?} - {:?} \n", len, &buffer[..len]);

        // search for the preambles (0xaa, 0x55)
        let h = search_header(&buffer[..len]).expect("Headers not found");
        eprintln!("h - {:?}, \n", h);

        //
        let p = divide_packet(&buffer[..len], &h).expect("Packets not found");
        for (i, c) in p.iter().enumerate() {
            eprintln!("p - {:?}/{:?}", i, c);
        }

        // sleep
        thread::sleep(Duration::from_millis(256));
    }
}

use itertools::Itertools;

pub fn search_header(buffer: &[u8]) -> Result<Vec<usize>> {
    let mut h = Vec::new();
    let buf = buffer.iter();
    for (i, c) in buf.enumerate().skip(1) {
        if buffer[i - 1] == 0xaa && buffer[i] == 0x55 {
            h.push(i - 1);
        }
    }
    Ok(h)
}

pub fn divide_packet<'a, 'b>(
    buffer: &'a [u8],
    h: &'b [usize],
) -> Result<Vec<std::slice::Iter<'a, u8>>> {
    //    Vec<&std::slice::Iter<'_, u8>>
    let mut p = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut b;
    for (i, c) in h.iter().enumerate() {
        if i + 1 != h.len() {
            start = h[i];
            end = h[i + 1];
        } else {
            start = h[i];
            end = buffer.len();
        }
        eprintln!("s/e - {:?}/{:?}", start, end);
        b = buffer[start..end].iter().clone();
        p.push(b);
    }
    Ok(p)
}
