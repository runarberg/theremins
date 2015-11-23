#![feature(convert)]

extern crate hyper;
extern crate ws;

use std::thread;

mod http_server;
mod ws_server;

fn main() {
    let http_host = arg_parse("--http-host", "0.0.0.0:8000");
    let ws_host = arg_parse("--ws-host", "0.0.0.0:8001");
    let ws_url = arg_parse("--ws-url", "localhost:8001");

    thread::spawn(move || ws_server::serve(&ws_host));
    thread::spawn(move || http_server::serve(&http_host, &ws_url));

    loop {}
}

fn arg_parse(opt: &str, default: &str) -> String {
    use std::process;

    match std::env::args().position(|s| s == opt) {
        Some(i) => if let Some(host) = std::env::args().nth(i + 1) {
            if host.starts_with("--") {
                println!(include_str!("../etc/usage.txt"));
                process::exit(1);
            }
            host
        } else {
            println!(include_str!("../etc/usage.txt"));
            process::exit(1);
        },
        None => default.to_string(),
    }
}
