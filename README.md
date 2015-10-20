Theremins
=========

Play the theremin with random strangers from the interet.

Run
---

### Server

```bash
git clone https://github.com/runarberg/theremins.git
cd server
cargo build --target release
./target/release/theremins
```

### Client

```bash
$BROWSER client/index.html
```

Have fun.

Note. Currently IE does not support web audio. But if you are using
IE10+ you should be able to play your theremin, but only people using
more modern browsers can hear it.
