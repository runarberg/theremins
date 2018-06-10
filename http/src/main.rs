extern crate docopt;
extern crate futures;
extern crate http;
extern crate hyper;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::env;

mod server;

const USAGE: &'static str = "
HTTP server for theremins.club

Usage:
  theremins-http-server [--port <port>] [--ws-url <url>]
  theremins-http-server --help

Options:
  -h --help       Show this screen
  --port <port>   HTTP address to listen to [env: THEREMINS_HTTP_PORT]
  --ws-url <url>  Web Socket URL to point to [env: THEREMINS_WS_URL].
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_port: Option<u16>,
    flag_ws_url: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let port = args.flag_port
        .or_else(|| {
            env::var("THEREMINS_HTTP_PORT")
                .ok()
                .and_then(|foo| foo.parse().ok())
        })
        .unwrap_or(8000);

    let ws_url = args.flag_ws_url
        .or_else(|| env::var("THEREMINS_WS_URL").ok())
        .unwrap_or("ws://localhost:8001".into());

    server::serve(port, ws_url);
}
