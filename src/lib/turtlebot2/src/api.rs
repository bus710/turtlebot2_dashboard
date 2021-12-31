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
        search_headers(&buffer[..len]);
        thread::sleep(Duration::from_millis(256));
    }
}

use itertools::Itertools;

pub fn search_headers(buffer: &[u8]) -> Result<u64> {
    let mut v = Vec::new();
    let itr = buffer.into_iter();
    for (i, c) in itr.enumerate().skip(1) {
        if buffer[i - 1] == 0xaa && buffer[i] == 0x55 {
            v.push(i);
        }
    }
    eprintln!("v - {:?}, \n", v);
    Ok(0)
}
