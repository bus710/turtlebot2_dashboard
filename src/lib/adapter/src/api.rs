#![allow(unused)]

// use std::os::raw;
// use std::slice::Iter;
// use std::sync::atomic::{AtomicI32, Ordering};
// use std::sync::{self, mpsc, Arc};
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Error, Result};
use crossbeam_channel::unbounded;
use itertools::Itertools;
use serialport::{SerialPortInfo, SerialPortType};

use ttb2::{rx, tx};
use turtlebot2 as ttb2;

use flutter_rust_bridge::rust2dart::TaskCallback;
use flutter_rust_bridge::{StreamSink, SyncReturn, ZeroCopyBuffer};

const SERIAL: &str = "kobuki";

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
