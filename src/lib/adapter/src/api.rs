#![allow(unused)]

// use std::os::raw;
// use std::slice::Iter;
// use std::sync::atomic::{AtomicI32, Ordering};
// use std::sync::{self, mpsc, Arc};
use std::convert::From;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Error, Result};
use crossbeam_channel::unbounded;
use itertools::Itertools;
use serialport::{SerialPortInfo, SerialPortType};

use ttb2::rx::Feedback;
use ttb2::{rx, tx};
use turtlebot2 as ttb2;

use flutter_rust_bridge::rust2dart::TaskCallback;
use flutter_rust_bridge::support::{into_leak_vec_ptr, DartCObject};
use flutter_rust_bridge::{StreamSink, SyncReturn, ZeroCopyBuffer};

const SERIAL: &str = "kobuki";

use once_cell::sync::OnceCell;
use std::sync::Mutex;

// Static channel to interact with adapter
static SEND: OnceCell<crossbeam_channel::Sender<bool>> = OnceCell::new();

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

pub fn spawn_adapter(sink: StreamSink<String>) -> Result<()> {
    let (sender, receiver) = unbounded();

    thread::spawn(move || loop {
        let a = receiver.recv().unwrap();
        println!("from Rust thread - {:?}", a);
        sink.add("a".to_string());
        thread::sleep(Duration::from_millis(1000));
    });

    // let g = Mutex::lock(SEND).unwrap();
    // let g = SEND.lock().unwrap();
    SEND.set(sender);
    Ok(())
}

pub fn send_to_adapter() -> Result<()> {
    let tx = SEND.get();
    let tx2 = tx.unwrap();
    tx2.send(true);
    Ok(())
}

pub fn receive_from_adapter() -> Result<()> {

    Ok(())
}
