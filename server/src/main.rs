#![feature(convert)]

extern crate ws;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ws::{listen, Handler, Sender, Result, Message, CloseCode, Error};

struct Server {
    out: Sender,
    sines: Rc<RefCell<HashMap<usize, String>>>
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(text) = msg.into_text() {
            (*self.sines.borrow_mut())
                .insert(self.out.token().as_usize(), text);
            let sines = self.sines.borrow();
            let json_seq = sines.values()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(",");
            self.out.broadcast(Message::text(format!("[{}]", json_seq)))
        } else {
            Ok(())
        }
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        let token = self.out.token();
        (*self.sines.borrow_mut())
            .remove(&token.as_usize());
    }

    fn on_error(&mut self, _: Error) {
        let token = self.out.token();
        (*self.sines.borrow_mut())
            .remove(&token.as_usize());
    }
}

fn main() {
    let host = "0.0.0.0:8001";
    let sines = Rc::new(RefCell::new(HashMap::new()));
    if let Err(error) = listen(host, |out| {
        Server {out: out, sines: sines.clone()}
    }) {
        println!("{:?}", error);
    };
}
