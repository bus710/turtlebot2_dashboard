mod api;
mod parser;

fn main() {
    api::hello2().expect("");
    api::enum_ports();
}
