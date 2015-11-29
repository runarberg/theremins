extern crate rustc_serialize;
extern crate docopt;
extern crate ws;

use docopt::Docopt;

mod server;

const USAGE: &'static str = "
Web socket server for theremins.club

Usage:
  theremins-ws-server [--address <address>]
  theremins-ws-server --help

Options:
  -h --help            Show this screen
  --address <address>  Web socket address to listen to [default: 0.0.0.0:8001]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_address: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    server::serve(&args.flag_address);
}
