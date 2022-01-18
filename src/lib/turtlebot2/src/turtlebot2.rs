use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use flutter_rust_bridge::StreamSink;

use crate::api::*;

#[derive(Clone)]
pub struct Turtlebot {
    receiver: crossbeam_channel::Receiver<bool>,
    sink: StreamSink<String>,
    // feedbacks: Vec<Feedback>,
}

impl Turtlebot {
    pub fn new(rx: crossbeam_channel::Receiver<bool>, sk: StreamSink<String>) -> Turtlebot {
        Turtlebot {
            receiver: rx,
            sink: sk,
            // feedbacks: Vec::new(),
        }
    }
    pub fn open(&mut self) {}
    pub fn close(&mut self) {}
    pub fn read(&mut self) {}
    pub fn write(&mut self) {}
}

pub struct TurtlebotRunner {
    turtlebot_lock: Arc<Mutex<Turtlebot>>,
}

impl TurtlebotRunner {
    pub fn new(rx: crossbeam_channel::Receiver<bool>, sk: StreamSink<String>) -> TurtlebotRunner {
        let ttb_lock = Turtlebot::new(rx, sk);
        TurtlebotRunner {
            turtlebot_lock: Arc::new(Mutex::new(ttb_lock)),
        }
    }

    pub fn run(&mut self) {
        // Get the mutex
        let ttb_lock = self.turtlebot_lock.clone();
        // Thread body
        thread::spawn(move || {
            // Unlock the mutex
            let mut ttb = ttb_lock.lock().unwrap();
            // Enter the loop
            loop {
                crossbeam_channel::select! {
                recv(ttb.receiver) -> v =>{
                    match v{
                        Ok(vv) => {
                            eprintln!("from Rust thread - {:?}", vv);
                            ttb.open();
                            ttb.close();
                            ttb.read();
                            ttb.write();
                            let f = Feedback::new();
                            let mut ff = Vec::new();
                            ff.push(f);
                        },
                        Err (e) => {
                            eprintln!("{:?}", e);},
                        }
                    }
                }
                ttb.sink.add("a".to_string());
                thread::sleep(Duration::from_millis(10));
            }
        });
    }
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
