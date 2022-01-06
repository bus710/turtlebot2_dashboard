#![allow(unused)]

use std::os::raw;
use std::slice::Iter;
// use std::sync::atomic::{AtomicI32, Ordering};
// use std::sync::{self, mpsc, Arc};
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
    turtlebot2::hello();

    Ok(())
}

pub fn enum_ports() -> Result<Vec<SerialPortInfo>> {
    let ports = serialport::available_ports().expect("No ports found!");
    Ok(ports)
}

pub fn open_port(port_name: String) {
    eprintln!("{:?}", port_name);

    let mut port = serialport::new(port_name, 115200)
        .open()
        .expect("Open port");
    port.set_timeout(Duration::from_millis(1024));

    let mut buffer = [0; 4096];
    let mut residue = Vec::new();

    for i in 0..10 {
        let len = port.read(&mut buffer).expect("Read failed");
        let (f, r) = turtlebot2::decode(len, &buffer, &residue).unwrap();
        eprintln!("f - {:?}", f);
        residue = r;

        eprintln!("==================");
        eprintln!();
        thread::sleep(Duration::from_millis(64)); // with 64 ms, the read returns about 220~350 bytes
    }
}
