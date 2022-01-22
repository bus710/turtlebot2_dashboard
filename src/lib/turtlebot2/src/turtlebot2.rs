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
use serialport::{SerialPort, SerialPortType, UsbPortInfo};

use crate::api::*;
use crate::rx::*;
use crate::tx::*;

// Keyword to find USB-Serial devices
const SERIAL: &str = "kobuki";
// Static channel to interact with turtlebot
static SEND: OnceCell<Arc<Mutex<crossbeam::Sender<Command>>>> = OnceCell::new();
// Static vector to store feedbacks from turtlebot
static RECEIVE: OnceCell<Arc<Mutex<Vec<Feedback>>>> = OnceCell::new();

// Will be called by the spawning function
pub fn set_statics_in_turtlebot(sender: crossbeam::Sender<Command>) {
    // The global static SEND is used to send command to the turtlebot instance
    let sender_lock = Arc::new(Mutex::new(sender));
    SEND.set(sender_lock);
    // The global static RECEIVE is used to receive feedbacks by Flutter
    let feedback_lock = Arc::new(Mutex::new(Vec::new()));
    RECEIVE.set(feedback_lock);
}

// To send commands to the thread in Turtlebot
pub fn send(cmd: Command) {
    let tx_lock = SEND.get().unwrap();
    let tx = tx_lock.lock().unwrap();
    tx.send(cmd);
}

// To read stored Feedbacks by Flutter
pub fn receive() -> Result<Vec<Feedback>> {
    let fbd_lock = RECEIVE.get().unwrap();
    let mut fbd = fbd_lock.lock().unwrap();
    if fbd.len() > 0 {
        let fbd_ = fbd.clone();
        fbd.clear();
        return Ok(fbd_.clone());
    }
    Err(anyhow!("What feedback?"))
}

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
            // Be careful! - these channels are twisted for bidirectional comm.
            // ttb_tx => serial_rx
            // serial_tx => ttb_rx
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
                    // Need to send back to indicate the port is opened
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
                                    // Need to send back to indicate the port is closed
                                    tx.send(Command {
                                        ty: CommandId::SerialControl,
                                        serial_command: "closed".to_string(),
                                        serial_port_name: serial_port_name.clone(),
                                        payload: Vec::new(),
                                    });
                                    // Exit from the loop so the port will be dropped
                                    break;
                                }
                            }
                            recv(ticker)-> _ => {
                                let r = p.read(&mut buffer);
                                let mut len = 0;
                                match r {
                                    Ok(l) => {len = l;}
                                    Err(e) => {
                                        eprintln!("read failed => exit");
                                        // Need to send back to indicate there is an issue
                                        tx.send(Command {
                                            ty: CommandId::SerialControl,
                                            serial_command: "error".to_string(),
                                            serial_port_name: "".to_string(),
                                            payload: Vec::new(),
                                        });
                                        break
                                    }
                                }
                                let d = decode(&buffer[..len], &residue);
                                match d {
                                    // Incoming packets are well decoded.
                                    // Push to the static vector
                                    Ok(v) => {
                                        let(mut f, r) = v;
                                        let fdb_lock = RECEIVE.get().unwrap();
                                        let mut fdb = fdb_lock.lock().unwrap();
                                        fdb.append(&mut f);
                                        let fdb_locak = Arc::new(Mutex::new(fdb));
                                        RECEIVE.set(fdb_lock.clone());
                                        residue = r;

                                        // Need to send back to indicate the feedbacks are ready
                                        // Then Flutter will receive and read the vector
                                        tx.send(Command {
                                            ty: CommandId::SerialControl,
                                            serial_command: "ready".to_string(),
                                            serial_port_name: "".to_string(),
                                            payload: Vec::new(),
                                        });
                                    }
                                    // If failed to decode? TBD
                                    Err(e) => {
                                    }
                                }
                            }
                        }
                    }
                }
                // Failed to open any port.
                // No state changes.
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
                        match c.serial_command.as_str() {
                            "opened" => {
                                ttb_data.current_port_opened = true;
                                ttb_data.current_port_name= c.serial_port_name;
                            }
                            "closed" => {
                                ttb_data.current_port_opened = false;
                                ttb_data.current_port_name= "".to_string();
                            }
                            "error" => {
                                ttb_data.current_port_opened = false;
                                ttb_data.current_port_name = "".to_string();
                            }
                            "ready" => {
                                ttb_data.sink.add("ready".to_string());
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }
}
