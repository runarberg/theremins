Theremins
=========

Play the theremin with random strangers from the interet.

Install
-------

This repo proviedes sources for two binaries, an HTTP server and a
web-socket server. To compile the HTTP server do:

```bash
git clone https://github.com/runarberg/theremins.git
cd theremins/http
cargo build --release
```

To compile the web-socket server simply swap line 2 above with:

```bash
cd theremins/ws
```

Run
---

Run the executibles are located in `target/release/`. For example to
run the web socket server on port `8001`

```bash
./theremins/ws/target/release/theremins-ws-server \
    --address 0.0.0.0:8001
```

For the HTTP server you have to specify where your web sockets server
lives, so you need, for example:

```bash
./theremins/ws/target/release/theremins-http-server \
    --address 0.0.0.0:8000 \
    --ws-url ws://localhost:8001
```

Have fun.

Note. Currently IE does not support web audio. But if you are using
IE10+ you should be able to play your theremin, but only people using
more modern browsers can hear it.
