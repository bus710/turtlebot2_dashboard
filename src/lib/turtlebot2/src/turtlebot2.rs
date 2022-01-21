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
