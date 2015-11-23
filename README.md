Theremins
=========

Play the theremin with random strangers from the interet.

Run
---

### Server

```bash
git clone https://github.com/runarberg/theremins.git
cd server
cargo build --release
./target/release/theremins-server --http-host 0.0.0.0:3000 --ws-host 0.0.0.0:3012
```

### Client

```bash
$BROWSER localhost:3000
```

Have fun.

Note. Currently IE does not support web audio. But if you are using
IE10+ you should be able to play your theremin, but only people using
more modern browsers can hear it.
