#![feature(convert)]

mod web_sockets;

fn main() {
    let host = "0.0.0.0:8001";
    web_sockets::server(host);
}
