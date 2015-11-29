extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;

use docopt::Docopt;

mod server;

const USAGE: &'static str = "
HTTP server for theremins.club

Usage:
  theremins-http-server [--address <address>] [--ws-url <url>]
  theremins-http-server --help

Options:
  -h --help            Show this screen.
  --address <address>  HTTP address to listen to [default: 0.0.0.0:8000].
  --ws-url <url>       Web Socket URL to point to [default: ws://localhost:8001].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_address: String,
    flag_ws_url: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    server::serve(&args.flag_address, &args.flag_ws_url);
}
