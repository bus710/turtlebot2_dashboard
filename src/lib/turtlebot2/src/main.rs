mod api;
mod parser;

const PORT: &str = "/dev/ttyUSB0";

fn main() {
    api::hello2().expect("Just to see the module hierarchy");

    let ports = api::enum_ports().expect("cannot enumerate ports");
    let mut found_ttyusb0 = false;
    if ports.len() < 1 {
        panic!("No USB serial devices found")
    }

    for p in ports.iter() {
        eprintln!("{:?} - {:?}", p.port_name, p);
        if p.port_name.contains(PORT) {
            found_ttyusb0 = true;
        }
    }

    if found_ttyusb0 {
        //
        api::open_port(String::from(PORT));
    }

    eprintln!("{:?}", ports);
}
