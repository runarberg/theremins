extern crate docopt;
extern crate ws;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::env;

mod server;

const USAGE: &'static str = "
Web socket server for theremins.club

Usage:
  theremins-ws-server [--port <port>]
  theremins-ws-server --help

Options:
  -h --help      Show this screen
  --port <port>  Web socket address to listen to [env: THEREMINS_WS_PORT]
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_port: Option<u16>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let port = args.flag_port
        .or_else(|| {
            env::var("THEREMINS_WS_PORT")
                .ok()
                .and_then(|foo| foo.parse().ok())
        })
        .unwrap_or(8001);

    server::serve(port);
}
