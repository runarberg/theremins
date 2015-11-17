#![feature(convert)]

extern crate hyper;
extern crate ws;

use std::thread;

mod http_server;
mod ws_server;

fn main() {
    let http_host = "0.0.0.0:8000";
    let ws_host = "0.0.0.0:8001";
    thread::spawn(move || ws_server::serve(ws_host));
    http_server::serve(http_host);
}
